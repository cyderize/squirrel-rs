use super::*;

extern {
	pub fn sqstd_register_systemlib(v: HSQUIRRELVM) -> SQInteger;
}