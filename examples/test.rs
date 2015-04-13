extern crate squirrel;

use squirrel::{SquirrelVM,};
use std::io::{Write, stdin, stdout, stderr};

fn main() {
	let mut squirrel = SquirrelVM::new(1024, stdout(), stderr());
	
	squirrel.push_root_table();
	
	squirrel.register_blob_lib().unwrap();
	squirrel.register_io_lib().unwrap();
	squirrel.register_system_lib().unwrap();
	squirrel.register_math_lib().unwrap();
	squirrel.register_string_lib().unwrap();
	
	squirrel.compile_str("seterrorhandler(function(x) { print(x + \"\\n\") })", "error_handler").unwrap();
	squirrel.push_root_table();
	squirrel.call(1, false, true).unwrap();
	squirrel.pop(1);
	
	let mut stdin = stdin();
	
	loop {
		print!(">");
		stdout().flush().unwrap();
	
		let mut line = String::new();
		stdin.read_line(&mut line).unwrap();
		
		if (&line[..]).trim() == "quit" {
			break;
		}
		
		match squirrel.compile_str(&line[..], "program") {
			Ok(()) => (),
			Err(e) => {
				println!("{}", e);
				squirrel.set_top(1);
				continue;
			}
		}
		squirrel.push_root_table();

		match squirrel.call(1, false, true) {
			Ok(()) => (),
			Err(()) => {
				squirrel.set_top(1);
				continue;
			}
		}
		squirrel.pop(1);
		
		println!("");
	}
	
	squirrel.pop(1);
}