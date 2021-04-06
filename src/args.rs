#![allow(unused_mut)]
use std::path::PathBuf;
use std::env;

const HELP: &'static str = r#"ripwc
William L. <williamlane923@gmail.com>

USAGE:
    ripwc [FLAGS] [file]

ARGS:
    <file>    Input file

FLAGS:
    -c               Show number of bytes
    -l               Show line count
    -m               Show number of characters
    -w               Show number of words
"#;

pub struct Opts {
	pub file: Option<PathBuf>,
	pub l: bool,
	pub w: bool,
	pub c: bool,
	pub m: bool,
}

impl Opts {
	pub fn parse() -> Self {
		let mut args = env::args();
		let mut file = None;
		let mut l = false;
		let mut w = false;
		let mut c = false;
		let mut m = false;

		match args.len() {
			1 => { print!("{}", HELP); std::process::exit(1) },
			_ => {
				args.next();
				match args.next() {
					Some(arg) => {
						if &arg != "-" {
							file = Some(PathBuf::from(arg))
						}
					},
					_ => ()
				}
			}
		}

		Self { file, l, w, c, m }
	}
}
