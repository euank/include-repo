#[macro_use]
extern crate include_repo;

use std::io::Write;

// This includes the source code of the current git repo (that is, the entire include-repo git
// repo) as a const named 'SOURCE_CODE'.
const SOURCE_CODE: &[u8] = include_repo!();

fn main() {
    let mut f = std::fs::File::create("/tmp/code.tar").expect("could not open file");
    f.write_all(&SOURCE_CODE[..]).expect("could not write file");
    println!(
        "The source code of this program has been written to {} for agpl compliance reasons",
        "/tmp/code.tar"
    );
}
