//! The squirrel crate provides bindings to the Squirrel language.

extern crate squirrel_sys as ffi;
extern crate libc;

use libc::c_char;
use std::marker::PhantomData;
use std::ffi::{CStr, CString};
use std::{ptr, mem};
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::str::from_utf8;
use std::slice;

/// Print shim callback type
pub type PrintFn = extern fn(v: ffi::HSQUIRRELVM, len: usize, buf: *const c_char);

extern {
	fn shim_set_print_callback(cb: PrintFn);
	fn shim_set_err_callback(cb: PrintFn);
	
	pub fn shim_print_fn(v: ffi::HSQUIRRELVM, s: *const ffi::SQChar, ...);
	pub fn shim_err_fn(v: ffi::HSQUIRRELVM, s: *const ffi::SQChar, ...);
}

/// Print callback
extern fn print_fn<P: Write, E: Write>(v: ffi::HSQUIRRELVM, len: usize, buf: *const c_char) {
	let buffer = unsafe { slice::from_raw_parts(buf as *const u8, len) };
	
	let data: &mut SquirrelData<P, E> = unsafe { mem::transmute(ffi::sq_getforeignptr(v)) };
	data.print.write(buffer).unwrap();
}

/// Error callback
extern fn err_fn<P: Write, E: Write>(v: ffi::HSQUIRRELVM, len: usize, buf: *const c_char) {
	let buffer = unsafe { slice::from_raw_parts(buf as *const u8, len) };
	
	let data: &mut SquirrelData<P, E> = unsafe { mem::transmute(ffi::sq_getforeignptr(v)) };
	data.error.write(buffer).unwrap();
}

/// Handles compiler errors
extern fn err_handler(v: ffi::HSQUIRRELVM, desc: *const ffi::SQChar, source: *const ffi::SQChar, line: ffi::SQInteger, column: ffi::SQInteger) {
	unsafe {
		let desc = from_utf8(CStr::from_ptr(desc).to_bytes()).unwrap().to_string();
		let source = from_utf8(CStr::from_ptr(source).to_bytes()).unwrap().to_string();

		let error: &mut Option<CompilerError> = mem::transmute(ffi::sq_getforeignptr(v));
		*error = Some(CompilerError {
			desc: desc,
			source: source,
			line: line,
			column: column
		});
	}
}

/// Reads from an Iterator over chars stored at a pointer
extern fn read_fn<C: Iterator<Item = char>>(ptr: ffi::SQUserPointer) -> ffi::SQInteger {
	let chars: &mut C = unsafe { mem::transmute(ptr) };
	let next = chars.next();
	
	match next {
		Some(c) => c as ffi::SQInteger,
		None => 0
	}
}

/// Represents data relevant to a Squirrel virtual machine
struct SquirrelData<P, E> {
	#[allow(dead_code)]
	compiler_error: Option<CompilerError>,
	print: P,
	error: E,
}

fn get_result<T, E>(r: ffi::SQRESULT, t: T, e: E) -> Result<T, E> {
	if ffi::SQ_SUCCEEDED(r) {
		Ok(t)
	}
	else {
		Err(e)
	}
}

pub fn get_version() -> isize {
	return unsafe { ffi::sq_getversion() } as isize;
}

/// Represents a Squirrel virtual machine.
pub struct SquirrelVM<P, E>(ffi::HSQUIRRELVM, PhantomData<(P, E)>);

impl<P: Write + Sync, E: Write + Sync> SquirrelVM<P, E> {
	/* VM functions */
	
	/// Create a new Squirrel virtual machine.
	///
	/// Takes an initial stack size, a stream to print to, and a stream to write errors to.
	/// # Example
	/// ```
	/// use std::io::{stdout, stderr};
	/// let vm = SquirrelVM::new(1024, stdout(), stderr());
	/// ```
	pub fn new(initial_stack: isize, print_stream: P, error_stream: E) -> SquirrelVM<P, E> {
		let data = Box::new(SquirrelData {
			print: print_stream,
			error: error_stream,
			compiler_error: None
		});
		
		let vm = unsafe { ffi::sq_open(initial_stack) };
		unsafe {
			// Turns the box into a raw pointer - the structure is freed when dropped
			ffi::sq_setforeignptr(vm, mem::transmute(data));
			ffi::sq_setprintfunc(vm, shim_print_fn, shim_err_fn);
			ffi::sq_setcompilererrorhandler(vm, err_handler);
			
			shim_set_print_callback(print_fn::<P, E>);
			shim_set_err_callback(err_fn::<P, E>);
		}
		SquirrelVM(vm, PhantomData)
	}
	/// Creates a new Squirrel virtual machine that is a friend of this machine.
	pub fn new_thread<Q, F>(&self, initial_stack: isize, print_stream: Q, error_stream: F) -> SquirrelVM<Q, F> {
		let data = Box::new(SquirrelData {
			print: print_stream,
			error: error_stream,
			compiler_error: None
		});
		
		let vm = unsafe { ffi::sq_newthread(self.0, initial_stack) };
		unsafe {
			// Turns the box into a raw pointer - the structure is freed when dropped
			ffi::sq_setforeignptr(vm, mem::transmute(data));
			ffi::sq_setprintfunc(vm, shim_print_fn, shim_err_fn);
			ffi::sq_setcompilererrorhandler(vm, err_handler);
			
			shim_set_print_callback(print_fn::<P, E>);
			shim_set_err_callback(err_fn::<P, E>);
		}
		SquirrelVM(vm, PhantomData)
	}
	/// Pops a function from the stack and sets it to be the runtime error handler.
	pub fn set_error_handler(&mut self) {
		unsafe {
			ffi::sq_seterrorhandler(self.0);
		}
	}
	pub fn suspend(&mut self) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_suspendvm(self.0)
		}, (), ())
	}
	pub fn wake_up(&mut self, resumed_return: bool, return_value: bool, raise_error: bool, throw_error: bool) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_wakeupvm(self.0, resumed_return as ffi::SQBool, return_value as ffi::SQBool, raise_error as ffi::SQBool, throw_error as ffi::SQBool)
		}, (), ())
	}
	pub fn get_vm_state(&self) -> State {
		match unsafe { ffi::sq_getvmstate(self.0) } {
			ffi::SQ_VMSTATE_IDLE => State::Idle,
			ffi::SQ_VMSTATE_RUNNING => State::Running,
			ffi::SQ_VMSTATE_SUSPENDED => State::Suspended,
			_ => unreachable!(),
		}
	}
	
	/* Compiler functions */
	
	/// Compiles a Squirrel script.
	///
	/// `chars` is an iterator over the characters in the script.
	/// `name` is symbolic name of the script (used to provide useful runtime debugging information).
	pub fn compile<C: Iterator<Item = char>>(&mut self, chars: &mut C, name: &str) -> Result<(), CompilerError> {
		let name = CString::new(name).unwrap();
		let result = unsafe {
			ffi::sq_compile(self.0, read_fn::<C>, mem::transmute(chars), mem::transmute(name.as_ptr()), 1)
		};
		
		if ffi::SQ_SUCCEEDED(result) {
			Ok(())
		}
		else {
			let data: &mut Option<CompilerError> = unsafe { mem::transmute(ffi::sq_getforeignptr(self.0)) };
			let error = data.clone().unwrap();
			*data = None;
			Err(error)
		}
	}
	/// Compiles a Squirrel script stored in a `&str`.
	///
	/// `src` is a `&str` containing the sourc code of the script.
	/// `name` is symbolic name of the script (used to provide useful runtime debugging information).
	pub fn compile_str(&mut self, src: &str, name: &str) -> Result<(), CompilerError> {
		let len = src.len();
		let src = CString::new(src).unwrap();
		let name = CString::new(name).unwrap();
		
		let result = unsafe {
			ffi::sq_compilebuffer(self.0, mem::transmute(src.as_ptr()), len as isize, mem::transmute(name.as_ptr()), 1)
		};
		
		if ffi::SQ_SUCCEEDED(result) {
			Ok(())
		}
		else {
			let data: &mut Option<CompilerError> = unsafe { mem::transmute(ffi::sq_getforeignptr(self.0)) };
			let error = data.clone().unwrap();
			*data = None;
			Err(error)
		}
	}
	/// Enables or disables debug info.
	pub fn set_debug_info(&mut self, enable: bool) {
		unsafe {
			ffi::sq_enabledebuginfo(self.0, enable as ffi::SQBool);
		}
	}
	/// Enables or disables notification of all exceptions.
	pub fn set_notify_all_exceptions(&mut self, enable: bool) {
		unsafe {
			ffi::sq_notifyallexceptions(self.0, enable as ffi::SQBool);
		}
	}
	
	/* Stack Functions */
	
	pub fn push(&mut self, idx: isize) {
		unsafe { ffi::sq_push(self.0, idx); }
	}
	
	pub fn pop(&mut self, element_count: isize) {
		unsafe { ffi::sq_pop(self.0, element_count); }
	}
	
	pub fn pop_top(&mut self) {
		unsafe { ffi::sq_poptop(self.0); }
	}
	
	pub fn remove(&mut self, idx: isize) {
		unsafe { ffi::sq_remove(self.0, idx); }
	}
	
	pub fn get_top(&self) -> isize {
		unsafe { ffi::sq_gettop(self.0) }
	}
	
	pub fn set_top(&mut self, new_top: isize) {
		unsafe { ffi::sq_settop(self.0, new_top); }
	}
	
	pub fn reserve_stack(&self, n_size: isize) {
		unsafe { ffi::sq_reservestack(self.0, n_size); }
	}
	
	pub fn cmp(&self) -> isize {
		unsafe { ffi::sq_cmp(self.0) }
	}
	
	pub fn move_item<Q, F>(&mut self, dest: &mut SquirrelVM<Q, F>, idx: isize) {
		unsafe { ffi::sq_move(dest.0, self.0, idx); }
	}
	
	/* Object creation handling */
	
	pub fn new_user_data<'a, T>(&mut self) -> &'a mut T {
		unsafe {
			mem::transmute(ffi::sq_newuserdata(self.0, mem::size_of::<T>() as ffi::SQUnsignedInteger))
		}
	}
	
	pub fn new_table(&mut self) {
		unsafe { ffi::sq_newtable(self.0); }
	}
	
	pub fn new_table_with_capacity(&mut self, capacity: usize) {
		unsafe { ffi::sq_newtableex(self.0, capacity as ffi::SQInteger); }
	}
	
	pub fn new_array(&mut self, size: usize) {
		unsafe { ffi::sq_newarray(self.0, size as ffi::SQInteger); }
	}
	
	pub fn new_closure<F: Fn(SquirrelVM<P, E>)>(&mut self, func: F, n_free_vars: usize) {
		// Can't think of how to do this yet
		unimplemented!();
	}
	
	pub fn set_params_check(&mut self, n_params_check: isize, type_mask: &str) -> Result<(), ()> {
		let type_mask = CString::new(type_mask).unwrap();
		get_result(unsafe {
			ffi::sq_setparamscheck(self.0, n_params_check as ffi::SQInteger, type_mask.as_ptr())
		}, (), ())
	}
	
	pub fn bind_env(&mut self, idx: isize) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_bindenv(self.0, idx)
		}, (), ())
	}
	
	pub fn push_str(&mut self, s: &str) {
		let s = CString::new(s).unwrap();
		unsafe { ffi::sq_pushstring(self.0, mem::transmute(s.as_ptr()), -1); }
	}
	
	pub fn push_float(&mut self, f: ffi::SQFloat) {
		unsafe { ffi::sq_pushfloat(self.0, f); }
	}
	
	pub fn push_integer(&mut self, n: ffi::SQInteger) {
		unsafe { ffi::sq_pushinteger(self.0, n); }
	}
	
	pub fn push_bool(&mut self, b: bool) {
		unsafe { ffi::sq_pushbool(self.0, b as ffi::SQBool); }
	}
	
	pub fn push_box<T>(&mut self, p: Box<T>) {
		unsafe { ffi::sq_pushuserpointer(self.0, mem::transmute(p)); }
	}
	
	pub fn push_null(&mut self) {
		unsafe { ffi::sq_pushnull(self.0); }
	}
	
	//pub fn sq_gettype(v: HSQUIRRELVM, idx: SQInteger) -> SQObjectType;
	//pub fn sq_typeof(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_getsize(v: HSQUIRRELVM, idx: SQInteger) -> SQInteger;
	//pub fn sq_gethash(v: HSQUIRRELVM, idx: SQInteger) -> SQHash;
	//pub fn sq_getbase(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_instanceof(v: HSQUIRRELVM) -> SQBool;
	//pub fn sq_tostring(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_tobool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> c_void;
	//pub fn sq_getstring(v: HSQUIRRELVM, idx: SQInteger, c: *mut *const SQChar) -> SQRESULT;
	//pub fn sq_getinteger(v: HSQUIRRELVM, idx: SQInteger, i: *mut SQInteger) -> SQRESULT;
	//pub fn sq_getfloat(v: HSQUIRRELVM, idx: SQInteger, f: *mut SQFloat) -> SQRESULT;
	//pub fn sq_getbool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> SQRESULT;
	//pub fn sq_getthread(v: HSQUIRRELVM, idx: SQInteger, thread: *mut HSQUIRRELVM) -> SQRESULT;
	//pub fn sq_getuserpointer(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer) -> SQRESULT;
	//pub fn sq_getuserdata(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer, typetag: *mut SQUserPointer) -> SQRESULT;
	//pub fn sq_settypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: SQUserPointer) -> SQRESULT;
	//pub fn sq_gettypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: *mut SQUserPointer) -> SQRESULT;
	//pub fn sq_setreleasehook(v: HSQUIRRELVM,idx: SQInteger, hook: SQRELEASEHOOK) -> c_void;
	//pub fn sq_getscratchpad(v: HSQUIRRELVM, minsize: SQInteger) -> *mut SQChar;
	//pub fn sq_getfunctioninfo(v: HSQUIRRELVM, level: SQInteger,  fi: *mut SQFunctionInfo) -> SQRESULT;
	//pub fn sq_getclosureinfo(v: HSQUIRRELVM, idx: SQInteger, nparams: *mut SQUnsignedInteger, nfreevars: *mut SQUnsignedInteger) -> SQRESULT;
	//pub fn sq_getclosurename(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_setnativeclosurename(v: HSQUIRRELVM, idx: SQInteger, name: *const SQChar) -> SQRESULT;
	//pub fn sq_setinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: SQUserPointer) -> SQRESULT;
	//pub fn sq_getinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer, typetag: SQUserPointer) -> SQRESULT;
	//pub fn sq_setclassudsize(v: HSQUIRRELVM, idx: SQInteger, udsize: SQInteger) -> SQRESULT;
	//pub fn sq_newclass(v: HSQUIRRELVM, hasbase: SQBool) -> SQRESULT;
	//pub fn sq_createinstance(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_setattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_getattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_getclass(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_weakref(v: HSQUIRRELVM, idx: SQInteger) -> c_void;
	//pub fn sq_getdefaultdelegate(v: HSQUIRRELVM, t: SQObjectType) -> SQRESULT;
	//pub fn sq_getmemberhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *mut HSQMEMBERHANDLE) -> SQRESULT;
	//pub fn sq_getbyhandle(v: HSQUIRRELVM,idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;
	//pub fn sq_setbyhandle(v: HSQUIRRELVM,idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;
	
	/* Object manipulation */
	
	pub fn push_root_table(&mut self) {
		unsafe { ffi::sq_pushroottable(self.0); }
	}

	pub fn push_registry_table(&mut self) {
		unsafe { ffi::sq_pushregistrytable(self.0); }
	}
	
	pub fn push_const_table(&mut self) {
		unsafe { ffi::sq_pushconsttable(self.0); }
	}
	
	pub fn set_root_table(&mut self) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_setroottable(self.0)
		}, (), ())
	}
	
	pub fn set_const_table(&mut self) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_setconsttable(self.0)
		}, (), ())
	}
	
	pub fn new_slot(&mut self, idx: isize, bstatic: bool) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_newslot(self.0, idx, bstatic as ffi::SQBool)
		}, (), ())
	}
	
	pub fn delete_slot(&mut self, idx: isize, push_val: bool) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_deleteslot(self.0, idx, push_val as ffi::SQBool)
		}, (), ())
	}
	
	pub fn set(&mut self, idx: isize) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_set(self.0, idx)
		}, (), ())
	}
	
	pub fn get(&mut self, idx: isize) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_get(self.0, idx)
		}, (), ())
	}
	
	//pub fn sq_rawget(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_rawset(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_rawdeleteslot(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
	//pub fn sq_newmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
	//pub fn sq_rawnewmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
	//pub fn sq_arrayappend(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_arraypop(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT; 
	//pub fn sq_arrayresize(v: HSQUIRRELVM, idx: SQInteger, newsize: SQInteger) -> SQRESULT; 
	//pub fn sq_arrayreverse(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT; 
	//pub fn sq_arrayremove(v: HSQUIRRELVM, idx: SQInteger, itemidx: SQInteger) -> SQRESULT;
	//pub fn sq_arrayinsert(v: HSQUIRRELVM, idx: SQInteger, destpos: SQInteger) -> SQRESULT;
	//pub fn sq_setdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_getdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_clone(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_setfreevariable(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> SQRESULT;
	//pub fn sq_next(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_getweakrefval(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	//pub fn sq_clear(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	
	/* Calls */
	
	pub fn call(&mut self, param_count: isize, retval: bool, raise_error: bool) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_call(self.0, param_count, retval as ffi::SQBool, raise_error as ffi::SQBool)
		}, (), ())
	}
	
	pub fn resume(&mut self, retval: bool, raise_error: bool) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_resume(self.0, retval as ffi::SQBool, raise_error as ffi::SQBool)
		}, (), ())
	}
	
	pub fn get_local(&mut self, level: usize, idx: usize) -> String {
		unsafe {
			from_utf8(CStr::from_ptr(ffi::sq_getlocal(self.0, level, idx)).to_bytes()).unwrap().to_string()
		}
	}
	
	pub fn get_callee(&mut self) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_getcallee(self.0)
		}, (), ())
	}
	
	pub fn get_free_variable(&mut self, idx: isize, nval: usize) -> String {
		unsafe {
			from_utf8(CStr::from_ptr(ffi::sq_getfreevariable(self.0, idx, nval)).to_bytes()).unwrap().to_string()
		}
	}
	
	pub fn throw_error(&mut self, error: &str) -> Result<(), ()> {
		let error = CString::new(error).unwrap();
		get_result(unsafe {
			ffi::sq_throwerror(self.0, error.as_ptr())
		}, (), ())
	}
	
	pub fn throw_object(&mut self) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_throwobject(self.0)
		}, (), ())
	}
	
	pub fn reset_error(&mut self) {
		unsafe {
			ffi::sq_reseterror(self.0);
		}
	}
	
	pub fn get_last_error(&mut self) {
		unsafe {
			ffi::sq_getlasterror(self.0);
		}
	}
	
	/* Raw object handling */
	
	//pub fn sq_getstackobj(v: HSQUIRRELVM, idx: SQInteger, po: *mut HSQOBJECT) -> SQRESULT;
	//pub fn sq_pushobject(v: HSQUIRRELVM, obj: HSQOBJECT) -> c_void;
	//pub fn sq_addref(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> c_void;
	//pub fn sq_release(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQBool;
	//pub fn sq_getrefcount(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQUnsignedInteger;
	//pub fn sq_resetobject(po: *mut HSQOBJECT) -> c_void;
	//pub fn sq_objtostring(o: *const HSQOBJECT) -> *const SQChar;
	//pub fn sq_objtobool(o: *const HSQOBJECT) -> SQBool;
	//pub fn sq_objtointeger(o: *const HSQOBJECT) -> SQInteger;
	//pub fn sq_objtofloat(o: *const HSQOBJECT) -> SQFloat;
	//pub fn sq_objtouserpointer(o: *const HSQOBJECT) -> SQUserPointer;
	//pub fn sq_getobjtypetag(o: *const HSQOBJECT, typetag: *mut SQUserPointer) -> SQRESULT;
	
	/* GC */
	
	pub fn collect_garbage(&mut self) -> isize {
		unsafe {
			ffi::sq_collectgarbage(self.0)
		}
	}
	
	pub fn resurrect_unreachable(&mut self) -> Result<(), ()> {
		get_result(unsafe {
			ffi::sq_resurrectunreachable(self.0)
		}, (), ())
	}
	
	/* Serialization */
	
	//pub fn write_closure(vm: HSQUIRRELVM, writef: SQWRITEFUNC, up: SQUserPointer) -> SQRESULT;
	//pub fn sq_readclosure(vm: HSQUIRRELVM, readf: SQREADFUNC,up: SQUserPointer) -> SQRESULT;
	
	/* Memory allocation */
	
	//pub fn sq_malloc(size: SQUnsignedInteger) -> *mut c_void;
	//pub fn sq_realloc(p: *mut c_void, oldsize: SQUnsignedInteger, newsize: SQUnsignedInteger) -> *mut c_void;
	//pub fn sq_free(p: *mut c_void, size: SQUnsignedInteger) -> c_void;
	
	/* Debug */
	
	pub fn stack_info(&self, level: isize) -> Result<StackInfo, ()> {
		let mut si = ffi::SQStackInfos {
			funcname: ptr::null(),
			source: ptr::null(),
			line: 0
		};
	
		unsafe { 
			if ffi::SQ_FAILED(ffi::sq_stackinfos(self.0, level, &mut si)) {
				return Err(());
			}
		}
	
		
		let func_name = if si.funcname.is_null() {
			"".to_string()
		}
		else {
			unsafe {
				from_utf8(CStr::from_ptr(si.funcname).to_bytes()).unwrap().to_string()
			}
		};
		
		let source = if si.source.is_null() {
			"".to_string()
		}
		else {
			unsafe {
				from_utf8(CStr::from_ptr(si.source).to_bytes()).unwrap().to_string()
			}
		};

		Ok(StackInfo {
			func_name: func_name,
			source: source,
			line: si.line
		})
	}
	
	pub fn set_debug_hook(&mut self) {
		unsafe { ffi::sq_setdebughook(self.0); }
	}
	
	//pub fn set_native_debug_hook<F: Fn(SquirrelVM, ...)>(&mut self, debug_hook: F) {
	
	//}
	
	/* stdlib */
	
	/// Registers the `stdblob` lib for use with this virtual machine.
	pub fn register_blob_lib(&mut self) -> Result<(), ()> {
		let result = unsafe { ffi::stdblob::sqstd_register_bloblib(self.0) };
		if ffi::SQ_SUCCEEDED(result) {
			Ok(())
		}
		else {
			Err(())
		}
	}
	/// Registers the `stdio` lib for use with this virtual machine.
	pub fn register_io_lib(&mut self) -> Result<(), ()> {
		let result = unsafe { ffi::stdio::sqstd_register_iolib(self.0) };
		if ffi::SQ_SUCCEEDED(result) {
			Ok(())
		}
		else {
			Err(())
		}
	}
	/// Registers the `stdmath` lib for use with this virtual machine.
	pub fn register_math_lib(&self) -> Result<(), ()> {
		let result = unsafe { ffi::stdmath::sqstd_register_mathlib(self.0) };
		if ffi::SQ_SUCCEEDED(result) {
			Ok(())
		}
		else {
			Err(())
		}
	}
	/// Registers the `stdstring` lib for use with this virtual machine.
	pub fn register_string_lib(&mut self) -> Result<(), ()> {
		let result = unsafe { ffi::stdstring::sqstd_register_stringlib(self.0) };
		if ffi::SQ_SUCCEEDED(result) {
			Ok(())
		}
		else {
			Err(())
		}
	}
	/// Registers the `stdsystem` lib for use with this virtual machine.
	pub fn register_system_lib(&mut self) -> Result<(), ()> {
		let result = unsafe { ffi::stdsystem::sqstd_register_systemlib(self.0) };
		if ffi::SQ_SUCCEEDED(result) {
			Ok(())
		}
		else {
			Err(())
		}
	}
}

impl<P, E> Drop for SquirrelVM<P, E> {
	fn drop(&mut self) {
		// Get a box so we free the memory
		let _: Box<SquirrelData<P, E>> = unsafe { mem::transmute(ffi::sq_getforeignptr(self.0)) };
		unsafe { ffi::sq_close(self.0) }
	}
}

/// Represents a compiler error thrown by a SquirrelVM.
#[derive(Debug, Clone)]
pub struct CompilerError {
	/// A description of the error.
	pub desc: String,
	/// The source name of the script that caused the error.
	pub source: String,
	/// The line the error is on.
	pub line: isize,
	/// The column the error is on.
	pub column: isize
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error: {} in '{}' on line '{}', column {}", self.desc, self.source, self.line, self.column)
	}
}

impl Error for CompilerError {
    fn description(&self) -> &str {
		&self.desc[..]
	}
}

/// Represents the state of a virtual machine
#[derive(Debug, Clone)]
pub enum State {
	Idle,
	Running,
	Suspended
}

/// Contains stack information for a virtual machine
#[derive(Debug, Clone)]
pub struct StackInfo {
	pub func_name: String,
	pub source: String,
	pub line: isize
}