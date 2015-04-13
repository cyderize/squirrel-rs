extern crate gcc;

fn main() {
	gcc::compile_library("libpshim.a", &["src/print_shim.c"]);
}