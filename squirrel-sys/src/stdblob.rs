use super::*;

extern {
	pub fn sqstd_createblob(v: HSQUIRRELVM, size: SQInteger) -> SQUserPointer;
	pub fn sqstd_getblob(v: HSQUIRRELVM, idx: SQInteger, ptr: *mut SQUserPointer) -> SQRESULT;
	pub fn sqstd_getblobsize(v: HSQUIRRELVM, idx: SQInteger) -> SQInteger;

	pub fn sqstd_register_bloblib(v: HSQUIRRELVM) -> SQRESULT;
}