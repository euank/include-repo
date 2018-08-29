#[macro_use]
extern crate include_repo;

use std::io::Write;

include_repo!(SOURCE_CODE);

fn main() {
	match std::env::args().skip(1).next() {
		Some(ref s) if s == "agpl" => {
			let mut f = std::fs::File::create("/tmp/code.tar").expect("could not open file");
			f.write_all(&SOURCE_CODE[..]).expect("could not write file");
			println!("The source code of this program has been written to {} for agpl compliance reasons", "/tmp/code.tar");
		},
		None => {
			println!("Hello world\nPlease pass 'agpl' as an argument to get my source code!");
		}
		_ => {
			println!("Unrecognized argument, I only know how to take 'agpl' as an argument to write out my code");
		}
	};
}
