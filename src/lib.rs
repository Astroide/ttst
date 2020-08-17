#[cfg(test)]
mod tests {
	use crate::util;
    #[test]
    fn intest() {
        util::input(">>>");
    }
}

pub mod constants {
	pub const VERSION: &'static str = "0.1.0";
	pub const LANGNAME: &'static str = "ttst";
}

pub mod util {
	use std::io::{self, Write};
	pub fn input(prompt: &str) -> String {
		let mut result = String::new();
		print!("{}", prompt);
		io::stdout().flush().expect(&format!("ERR ({}::util::input with prompt {} while flushing stdout)", crate::constants::LANGNAME, prompt)[..]);
		io::stdin().read_line(&mut result).expect(&format!("ERR ({}::util::input with prompt {} while calling read_line on stdin)", crate::constants::LANGNAME, prompt)[..]);
		result
	}
}

pub mod cmd_line {
	pub fn start(){
		use std::env::args as arg_fn;
		let mut args = arg_fn();
		if args.len()==1 {
			crate::cmd_line::repl();
		}else {
			crate::cmd_line::load_file(args.nth(1).expect("ERR arg 2"));
		}
	}
	pub fn repl(){
		use crate::util::input;
		loop {
			let tmp_user_input = input(">>>");
			let user_input = tmp_user_input.trim();
			if user_input=="^EXIT" {break}
			println!("<{}>", user_input);
		}
	}
	pub fn load_file(path: String) {
		unimplemented!();
	}
}