fn main() {
	// Link to squirrel and its standard library
	println!("cargo:rustc-flags=-l squirrel -l sqstdlib -l stdc++");
}