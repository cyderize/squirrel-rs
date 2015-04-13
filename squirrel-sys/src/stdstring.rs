use super::*;
use libc::c_uint;

pub type SQRexBool = c_uint;
#[repr(C)] pub struct SQRex;

#[repr(C)]
pub struct SQRexMatch {
	begin: *const SQChar,
	len: SQInteger
}

extern {
	pub fn sqstd_rex_compile(pattern: *const SQChar, error: *mut *const SQChar) -> *mut SQRex;
	pub fn sqstd_rex_free(exp: *mut SQRex);
	pub fn sqstd_rex_match(exp: *mut SQRex, text: *const SQChar) -> SQBool;
	pub fn sqstd_rex_search(exp: *mut SQRex, text: *const SQChar, out_begin: *mut *const SQChar, out_end: *mut *const SQChar) -> SQBool;
	pub fn sqstd_rex_searchrange(exp: *mut SQRex, text_begin: *const SQChar, text_end: *const SQChar, out_begin: *mut *const SQChar, out_end: *mut *const SQChar) -> SQBool;
	pub fn sqstd_rex_getsubexpcount(exp: *mut SQRex) -> SQInteger;
	pub fn sqstd_rex_getsubexp(exp: *mut SQRex, n: SQInteger, subexp: *mut SQRexMatch) -> SQBool;

	pub fn sqstd_format(v: HSQUIRRELVM, nformatstringidx: SQInteger, outlen: *mut SQInteger, output: *mut *mut SQChar) -> SQRESULT;

	pub fn sqstd_register_stringlib(v: HSQUIRRELVM) -> SQRESULT;
}