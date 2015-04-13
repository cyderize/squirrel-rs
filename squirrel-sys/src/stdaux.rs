use super::*;

extern {
	pub fn sqstd_seterrorhandlers(v: HSQUIRRELVM);
	pub fn sqstd_printcallstack(v: HSQUIRRELVM);
}