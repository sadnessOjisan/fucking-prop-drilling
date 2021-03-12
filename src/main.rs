use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let mut file = File::create("foo.txt").unwrap();
    writeln!(&mut file, "Hello World!").unwrap();
}
