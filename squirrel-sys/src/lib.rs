#![allow(improper_ctypes, non_snake_case, non_camel_case_types, unused_imports)]

//! This library provides bindings to the squirrel language.
//!
//! In addition, two cargo features are present:
//! `double-precision` and `wide-chars`
//! which require compilation of the squirrel library
//! with `SQUSEDOUBLE` and `SQUNICODE` respectively.

extern crate libc;

use libc::{c_void, c_char, c_ushort, wchar_t};

pub mod stdaux;
pub mod stdblob;
pub mod stdio;
pub mod stdmath;
pub mod stdstring;
pub mod stdsystem;

pub type SQInteger = isize;
pub type SQUnsignedInteger = usize;
pub type SQHash = usize;
pub type SQInt32 = i32; 
pub type SQUnsignedInteger32 = u32;

#[cfg(feature = "double-precision")]
pub type SQFloat = f64;
#[cfg(not(feature = "double-precision"))]
pub type SQFloat = f32;

#[cfg(any(all(feature = "double-precision", target_pointer_width = "32"), all(not(feature = "double-precision"), target_pointer_width = "64")))]
pub type SQRawObjectVal = i64;
#[cfg(not(any(all(feature = "double-precision", target_pointer_width = "32"), all(not(feature = "double-precision"), target_pointer_width = "64"))))]
pub type SQRawObjectVal = SQUnsignedInteger;

pub type SQUserPointer = *mut c_void;
pub type SQBool = SQUnsignedInteger;
pub type SQRESULT = SQInteger;

#[repr(C)] pub struct SQVM;
#[repr(C)] pub struct SQTable;
#[repr(C)] pub struct SQArray;
#[repr(C)] pub struct SQString;
#[repr(C)] pub struct SQClosure;
#[repr(C)] pub struct SQGenerator;
#[repr(C)] pub struct SQNativeClosure;
#[repr(C)] pub struct SQUserData;
#[repr(C)] pub struct SQFunctionProto;
#[repr(C)] pub struct SQRefCounted;
#[repr(C)] pub struct SQClass;
#[repr(C)] pub struct SQInstance;
#[repr(C)] pub struct SQDelegable;
#[repr(C)] pub struct SQOuter;

#[cfg(feature = "wide-chars")]
pub type SQChar = wchar_t;
#[cfg(not(feature = "wide-chars"))]
pub type SQChar = c_char;

pub const SQ_OK: SQRESULT = 0;
pub const SQ_ERROR: SQRESULT = -1;

pub fn SQ_FAILED(res: SQRESULT) -> bool { res < 0 }
pub fn SQ_SUCCEEDED(res: SQRESULT) -> bool {res >= 0}

pub const SQ_VMSTATE_IDLE: isize = 0;
pub const SQ_VMSTATE_RUNNING: isize = 1;
pub const SQ_VMSTATE_SUSPENDED: isize = 2;

pub const SQUIRREL_EOB: isize = 0;
pub const SQ_BYTECODE_STREAM_TAG: isize = 0xFAFA;

pub const SQOBJECT_REF_COUNTED: isize = 0x08000000;
pub const SQOBJECT_NUMERIC: isize = 0x04000000;
pub const SQOBJECT_DELEGABLE: isize = 0x02000000;
pub const SQOBJECT_CANBEFALSE: isize = 0x01000000;

pub const SQ_MATCHTYPEMASKSTRING: isize = -99999;

pub const _RT_MASK: isize = 0x00FFFFFF;
pub fn _RAW_TYPE(kind: isize) -> isize { kind & _RT_MASK }

pub const _RT_NULL: isize = 0x00000001;
pub const _RT_INTEGER: isize = 0x00000002;
pub const _RT_FLOAT: isize = 0x00000004;
pub const _RT_BOOL: isize = 0x00000008;
pub const _RT_STRING: isize = 0x00000010;
pub const _RT_TABLE: isize = 0x00000020;
pub const _RT_ARRAY: isize = 0x00000040;
pub const _RT_USERDATA: isize = 0x00000080;
pub const _RT_CLOSURE: isize = 0x00000100;
pub const _RT_NATIVECLOSURE: isize = 0x00000200;
pub const _RT_GENERATOR: isize = 0x00000400;
pub const _RT_USERPOINTER: isize = 0x00000800;
pub const _RT_THREAD: isize = 0x00001000;
pub const _RT_FUNCPROTO: isize = 0x00002000;
pub const _RT_CLASS: isize = 0x00004000;
pub const _RT_INSTANCE: isize = 0x00008000;
pub const _RT_WEAKREF: isize = 0x00010000;
pub const _RT_OUTER: isize = 0x00020000;

#[repr(C)]
pub enum SQObjectType {
	OT_NULL =			(_RT_NULL|SQOBJECT_CANBEFALSE),
	OT_INTEGER =		(_RT_INTEGER|SQOBJECT_NUMERIC|SQOBJECT_CANBEFALSE),
	OT_FLOAT =			(_RT_FLOAT|SQOBJECT_NUMERIC|SQOBJECT_CANBEFALSE),
	OT_BOOL =			(_RT_BOOL|SQOBJECT_CANBEFALSE),
	OT_STRING =			(_RT_STRING|SQOBJECT_REF_COUNTED),
	OT_TABLE =			(_RT_TABLE|SQOBJECT_REF_COUNTED|SQOBJECT_DELEGABLE),
	OT_ARRAY =			(_RT_ARRAY|SQOBJECT_REF_COUNTED),
	OT_USERDATA =		(_RT_USERDATA|SQOBJECT_REF_COUNTED|SQOBJECT_DELEGABLE),
	OT_CLOSURE =		(_RT_CLOSURE|SQOBJECT_REF_COUNTED),
	OT_NATIVECLOSURE =	(_RT_NATIVECLOSURE|SQOBJECT_REF_COUNTED),
	OT_GENERATOR =		(_RT_GENERATOR|SQOBJECT_REF_COUNTED),
	OT_USERPOINTER =	_RT_USERPOINTER,
	OT_THREAD =			(_RT_THREAD|SQOBJECT_REF_COUNTED) ,
	OT_FUNCPROTO =		(_RT_FUNCPROTO|SQOBJECT_REF_COUNTED), //internal usage only
	OT_CLASS =			(_RT_CLASS|SQOBJECT_REF_COUNTED),
	OT_INSTANCE =		(_RT_INSTANCE|SQOBJECT_REF_COUNTED|SQOBJECT_DELEGABLE),
	OT_WEAKREF =		(_RT_WEAKREF|SQOBJECT_REF_COUNTED),
	OT_OUTER =			(_RT_OUTER|SQOBJECT_REF_COUNTED) //internal usage only
}

#[repr(C)]
pub struct SQObjectValue {
	pub raw: SQRawObjectVal
}

#[repr(C)]
pub struct SQObject {
	pub _type: SQObjectType,
	pub _unVal: SQObjectValue
}

#[repr(C)]
pub struct SQMemberHandle {
	pub _static: SQBool,
	pub _index: SQInteger
}

#[repr(C)]
pub struct SQStackInfos {
	pub funcname: *const SQChar,
	pub source: *const SQChar,
	pub line: SQInteger
}

pub type HSQUIRRELVM = *mut SQVM;
pub type HSQOBJECT = SQObject;
pub type HSQMEMBERHANDLE = SQMemberHandle;
pub type SQFUNCTION = extern fn(HSQUIRRELVM) -> SQInteger;
pub type SQRELEASEHOOK = extern fn(SQUserPointer, size: SQInteger) -> SQInteger;
pub type SQCOMPILERERROR = extern fn(HSQUIRRELVM, *const SQChar /*desc*/, *const SQChar /*source*/, SQInteger /*line*/, SQInteger /*column*/);
pub type SQPRINTFUNCTION = unsafe extern fn(HSQUIRRELVM, *const SQChar, ...); // Unsafe as you can never define a non ffi function with this signature
pub type SQDEBUGHOOK = extern fn(HSQUIRRELVM /*v*/, SQInteger /*type*/, *const SQChar /*sourcename*/, SQInteger /*line*/, *const SQChar /*funcname*/);
pub type SQWRITEFUNC = extern fn(SQUserPointer,SQUserPointer,SQInteger) -> SQInteger;
pub type SQREADFUNC = extern fn(SQUserPointer,SQUserPointer,SQInteger) -> SQInteger;

pub type SQLEXREADFUNC = extern fn(SQUserPointer) -> SQInteger;

#[repr(C)]
pub struct SQRegFunction {
	pub name: *const SQChar,
	pub f: SQFUNCTION,
	pub nparamscheck: SQInteger,
	pub typemask: *const SQChar
}

#[repr(C)]
pub struct SQFunctionInfo {
	pub funcid: SQUserPointer,
	pub name: *const SQChar,
	pub source: *const SQChar
}

extern {
	/*vm*/
	pub fn sq_open(initialstacksize: SQInteger) -> HSQUIRRELVM;
	pub fn sq_newthread(friendvm: HSQUIRRELVM, initialstacksize: SQInteger) -> HSQUIRRELVM;
	pub fn sq_seterrorhandler(v: HSQUIRRELVM);
	pub fn sq_close(v: HSQUIRRELVM);
	pub fn sq_setforeignptr(v: HSQUIRRELVM, p: SQUserPointer);
	pub fn sq_getforeignptr(v: HSQUIRRELVM) -> SQUserPointer;
	pub fn sq_setprintfunc(v: HSQUIRRELVM, printfunc: SQPRINTFUNCTION, errfunc: SQPRINTFUNCTION);
	pub fn sq_getprintfunc(v: HSQUIRRELVM) -> SQPRINTFUNCTION;
	pub fn sq_geterrorfunc(v: HSQUIRRELVM) -> SQPRINTFUNCTION;
	pub fn sq_suspendvm(v: HSQUIRRELVM) -> SQRESULT;
	pub fn sq_wakeupvm(v: HSQUIRRELVM, resumedret: SQBool, retval: SQBool, raiseerror: SQBool, throwerror: SQBool) -> SQRESULT;
	pub fn sq_getvmstate(v: HSQUIRRELVM) -> SQInteger;
	pub fn sq_getversion() -> SQInteger;

	/*compiler*/
	pub fn sq_compile(v: HSQUIRRELVM, read: SQLEXREADFUNC, p: SQUserPointer,sourcename: *const SQChar, raiseerror: SQBool) -> SQRESULT;
	pub fn sq_compilebuffer(v: HSQUIRRELVM, s: *const SQChar, size: SQInteger, sourcename: *const SQChar, raiseerror: SQBool) -> SQRESULT;
	pub fn sq_enabledebuginfo(v: HSQUIRRELVM, enable: SQBool) -> c_void;
	pub fn sq_notifyallexceptions(v: HSQUIRRELVM, enable: SQBool) -> c_void;
	pub fn sq_setcompilererrorhandler(v: HSQUIRRELVM, f: SQCOMPILERERROR) -> c_void;

	/*stack operations*/
	pub fn sq_push(v: HSQUIRRELVM, idx: SQInteger) -> c_void;
	pub fn sq_pop(v: HSQUIRRELVM, nelemstopop: SQInteger) -> c_void;
	pub fn sq_poptop(v: HSQUIRRELVM) -> c_void;
	pub fn sq_remove(v: HSQUIRRELVM, idx: SQInteger) -> c_void;
	pub fn sq_gettop(v: HSQUIRRELVM) -> SQInteger;
	pub fn sq_settop(v: HSQUIRRELVM, newtop: SQInteger) -> c_void;
	pub fn sq_reservestack(v: HSQUIRRELVM, nsize: SQInteger) -> SQRESULT;
	pub fn sq_cmp(v: HSQUIRRELVM) -> SQInteger;
	pub fn sq_move(dest: HSQUIRRELVM, src: HSQUIRRELVM, idx: SQInteger) -> c_void;

	/*object creation handling*/
	pub fn sq_newuserdata(v: HSQUIRRELVM, size: SQUnsignedInteger) -> SQUserPointer;
	pub fn sq_newtable(v: HSQUIRRELVM) -> c_void;
	pub fn sq_newtableex(v: HSQUIRRELVM, initialcapacity: SQInteger) -> c_void;
	pub fn sq_newarray(v: HSQUIRRELVM, size: SQInteger) -> c_void;
	pub fn sq_newclosure(v: HSQUIRRELVM, func: SQFUNCTION, nfreevars: SQUnsignedInteger) -> c_void;
	pub fn sq_setparamscheck(v: HSQUIRRELVM, nparamscheck: SQInteger, typemask: *const SQChar) -> SQRESULT;
	pub fn sq_bindenv(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_pushstring(v: HSQUIRRELVM, s: *const SQChar, len: SQInteger) -> c_void;
	pub fn sq_pushfloat(v: HSQUIRRELVM, f: SQFloat) -> c_void;
	pub fn sq_pushinteger(v: HSQUIRRELVM, n: SQInteger) -> c_void;
	pub fn sq_pushbool(v: HSQUIRRELVM, b: SQBool) -> c_void;
	pub fn sq_pushuserpointer(v: HSQUIRRELVM, p: SQUserPointer) -> c_void;
	pub fn sq_pushnull(v: HSQUIRRELVM) -> c_void;
	pub fn sq_gettype(v: HSQUIRRELVM, idx: SQInteger) -> SQObjectType;
	pub fn sq_typeof(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_getsize(v: HSQUIRRELVM, idx: SQInteger) -> SQInteger;
	pub fn sq_gethash(v: HSQUIRRELVM, idx: SQInteger) -> SQHash;
	pub fn sq_getbase(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_instanceof(v: HSQUIRRELVM) -> SQBool;
	pub fn sq_tostring(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_tobool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> c_void;
	pub fn sq_getstring(v: HSQUIRRELVM, idx: SQInteger, c: *mut *const SQChar) -> SQRESULT;
	pub fn sq_getinteger(v: HSQUIRRELVM, idx: SQInteger, i: *mut SQInteger) -> SQRESULT;
	pub fn sq_getfloat(v: HSQUIRRELVM, idx: SQInteger, f: *mut SQFloat) -> SQRESULT;
	pub fn sq_getbool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> SQRESULT;
	pub fn sq_getthread(v: HSQUIRRELVM, idx: SQInteger, thread: *mut HSQUIRRELVM) -> SQRESULT;
	pub fn sq_getuserpointer(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer) -> SQRESULT;
	pub fn sq_getuserdata(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer, typetag: *mut SQUserPointer) -> SQRESULT;
	pub fn sq_settypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: SQUserPointer) -> SQRESULT;
	pub fn sq_gettypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: *mut SQUserPointer) -> SQRESULT;
	pub fn sq_setreleasehook(v: HSQUIRRELVM,idx: SQInteger, hook: SQRELEASEHOOK) -> c_void;
	pub fn sq_getscratchpad(v: HSQUIRRELVM, minsize: SQInteger) -> *mut SQChar;
	pub fn sq_getfunctioninfo(v: HSQUIRRELVM, level: SQInteger,  fi: *mut SQFunctionInfo) -> SQRESULT;
	pub fn sq_getclosureinfo(v: HSQUIRRELVM, idx: SQInteger, nparams: *mut SQUnsignedInteger, nfreevars: *mut SQUnsignedInteger) -> SQRESULT;
	pub fn sq_getclosurename(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_setnativeclosurename(v: HSQUIRRELVM, idx: SQInteger, name: *const SQChar) -> SQRESULT;
	pub fn sq_setinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: SQUserPointer) -> SQRESULT;
	pub fn sq_getinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer, typetag: SQUserPointer) -> SQRESULT;
	pub fn sq_setclassudsize(v: HSQUIRRELVM, idx: SQInteger, udsize: SQInteger) -> SQRESULT;
	pub fn sq_newclass(v: HSQUIRRELVM, hasbase: SQBool) -> SQRESULT;
	pub fn sq_createinstance(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_setattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_getattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_getclass(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_weakref(v: HSQUIRRELVM, idx: SQInteger) -> c_void;
	pub fn sq_getdefaultdelegate(v: HSQUIRRELVM, t: SQObjectType) -> SQRESULT;
	pub fn sq_getmemberhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *mut HSQMEMBERHANDLE) -> SQRESULT;
	pub fn sq_getbyhandle(v: HSQUIRRELVM,idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;
	pub fn sq_setbyhandle(v: HSQUIRRELVM,idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;

	/*object manipulation*/
	pub fn sq_pushroottable(v: HSQUIRRELVM) -> c_void;
	pub fn sq_pushregistrytable(v: HSQUIRRELVM) -> c_void;
	pub fn sq_pushconsttable(v: HSQUIRRELVM) -> c_void;
	pub fn sq_setroottable(v: HSQUIRRELVM) -> SQRESULT;
	pub fn sq_setconsttable(v: HSQUIRRELVM) -> SQRESULT;
	pub fn sq_newslot(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
	pub fn sq_deleteslot(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
	pub fn sq_set(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_get(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_rawget(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_rawset(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_rawdeleteslot(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
	pub fn sq_newmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
	pub fn sq_rawnewmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
	pub fn sq_arrayappend(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_arraypop(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT; 
	pub fn sq_arrayresize(v: HSQUIRRELVM, idx: SQInteger, newsize: SQInteger) -> SQRESULT; 
	pub fn sq_arrayreverse(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT; 
	pub fn sq_arrayremove(v: HSQUIRRELVM, idx: SQInteger, itemidx: SQInteger) -> SQRESULT;
	pub fn sq_arrayinsert(v: HSQUIRRELVM, idx: SQInteger, destpos: SQInteger) -> SQRESULT;
	pub fn sq_setdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_getdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_clone(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_setfreevariable(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> SQRESULT;
	pub fn sq_next(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_getweakrefval(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
	pub fn sq_clear(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;

	/*calls*/
	pub fn sq_call(v: HSQUIRRELVM, params: SQInteger, retval: SQBool, raiseerror: SQBool) -> SQRESULT;
	pub fn sq_resume(v: HSQUIRRELVM, retval: SQBool, raiseerror: SQBool) -> SQRESULT;
	pub fn sq_getlocal(v: HSQUIRRELVM, level: SQUnsignedInteger, idx: SQUnsignedInteger) -> *const SQChar;
	pub fn sq_getcallee(v: HSQUIRRELVM) -> SQRESULT;
	pub fn sq_getfreevariable(v: HSQUIRRELVM,idx: SQInteger, nval: SQUnsignedInteger) -> *const SQChar;
	pub fn sq_throwerror(v: HSQUIRRELVM, err: *const SQChar) -> SQRESULT;
	pub fn sq_throwobject(v: HSQUIRRELVM) -> SQRESULT;
	pub fn sq_reseterror(v: HSQUIRRELVM) -> c_void;
	pub fn sq_getlasterror(v: HSQUIRRELVM) -> c_void;

	/*raw object handling*/
	pub fn sq_getstackobj(v: HSQUIRRELVM, idx: SQInteger, po: *mut HSQOBJECT) -> SQRESULT;
	pub fn sq_pushobject(v: HSQUIRRELVM, obj: HSQOBJECT) -> c_void;
	pub fn sq_addref(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> c_void;
	pub fn sq_release(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQBool;
	pub fn sq_getrefcount(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQUnsignedInteger;
	pub fn sq_resetobject(po: *mut HSQOBJECT) -> c_void;
	pub fn sq_objtostring(o: *const HSQOBJECT) -> *const SQChar;
	pub fn sq_objtobool(o: *const HSQOBJECT) -> SQBool;
	pub fn sq_objtointeger(o: *const HSQOBJECT) -> SQInteger;
	pub fn sq_objtofloat(o: *const HSQOBJECT) -> SQFloat;
	pub fn sq_objtouserpointer(o: *const HSQOBJECT) -> SQUserPointer;
	pub fn sq_getobjtypetag(o: *const HSQOBJECT, typetag: *mut SQUserPointer) -> SQRESULT;

	/*GC*/
	pub fn sq_collectgarbage(v: HSQUIRRELVM) -> SQInteger;
	pub fn sq_resurrectunreachable(v: HSQUIRRELVM) -> SQRESULT;

	/*serialization*/
	pub fn sq_writeclosure(vm: HSQUIRRELVM, writef: SQWRITEFUNC, up: SQUserPointer) -> SQRESULT;
	pub fn sq_readclosure(vm: HSQUIRRELVM, readf: SQREADFUNC,up: SQUserPointer) -> SQRESULT;

	/*mem allocation*/
	pub fn sq_malloc(size: SQUnsignedInteger) -> *mut c_void;
	pub fn sq_realloc(p: *mut c_void, oldsize: SQUnsignedInteger, newsize: SQUnsignedInteger) -> *mut c_void;
	pub fn sq_free(p: *mut c_void, size: SQUnsignedInteger) -> c_void;

	/*debug*/
	pub fn sq_stackinfos(v: HSQUIRRELVM, level: SQInteger, si: *mut SQStackInfos) -> SQRESULT;
	pub fn sq_setdebughook(v: HSQUIRRELVM) -> c_void;
	pub fn sq_setnativedebughook(v: HSQUIRRELVM, hook: SQDEBUGHOOK) -> c_void;
	
}
