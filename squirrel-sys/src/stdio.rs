use super::*;
use libc::c_void;

pub type SQFILE = *mut c_void;

#[repr(C)]
pub struct SQStream {
	pub Read: extern fn(buffer: *mut c_void, size: SQInteger) -> SQInteger,
	pub Write: extern fn(buffer: *mut c_void, size: SQInteger) -> SQInteger,
	pub Flush: extern fn() -> SQInteger,
	pub Tell: extern fn() -> SQInteger,
	pub Len: extern fn() -> SQInteger,
	pub Seek: extern fn(offset: SQInteger, origin: SQInteger) -> SQInteger,
	pub IsValid: extern fn() -> bool,
	pub EOS: extern fn() -> bool
}

extern {
	pub fn sqstd_fopen(f: *const SQChar, m: *const SQChar) -> SQFILE;
	pub fn sqstd_fread(ptr: SQUserPointer, size: SQInteger, nmemb: SQInteger, f: SQFILE) -> SQInteger;
	pub fn sqstd_fwrite(ptr: SQUserPointer, size: SQInteger, nmemb: SQInteger, f: SQFILE) -> SQInteger;
	pub fn sqstd_fseek(f: SQFILE, offset: SQInteger, whence: SQInteger) -> SQInteger;
	pub fn sqstd_ftell(f: SQFILE) -> SQInteger;
	pub fn sqstd_fflush(f: SQFILE) -> SQInteger;
	pub fn sqstd_fclose(f: SQFILE) -> SQInteger;
	pub fn sqstd_feof(f: SQFILE) -> SQInteger;

	pub fn sqstd_createfile(v: HSQUIRRELVM, file: SQFILE, own: SQBool) -> SQRESULT;
	pub fn sqstd_getfile(v: HSQUIRRELVM, idx: SQInteger, file: *mut SQFILE) -> SQRESULT;

	// Compiler helpers
	pub fn sqstd_loadfile(v: HSQUIRRELVM,filename: *const SQChar, printerror: SQBool) -> SQRESULT;
	pub fn sqstd_dofile(v: HSQUIRRELVM,filename: *const SQChar, retval:SQBool, printerror: SQBool) -> SQRESULT;
	pub fn sqstd_writeclosuretofile(v: HSQUIRRELVM, filename: *const SQChar) -> SQRESULT;

	pub fn sqstd_register_iolib(v: HSQUIRRELVM) -> SQRESULT;
}