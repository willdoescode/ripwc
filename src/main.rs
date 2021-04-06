#[allow(unused_imports)]
use std::path::PathBuf;
use clap::Clap;
use args::Opts;
mod args;
use std::io::Read;
use std::fs::File;
use std::error::Error;

fn main() {
	let opts: Opts = Opts::parse();
	match get_input(&opts) {
		Ok(input) => {
			if opts.c {
				println!("Total bytes in file: {}", input.as_bytes().len())
			}
		},
		Err(err) => {
			eprintln!("{}", err.to_string());
			std::process::exit(1);
		},
	}
}

fn get_input(opts: &Opts) -> Result<String, Box<dyn Error>> {
	let mut input = String::new();

	if let Some(file) = &opts.file {
		File::open(&file)?.read_to_string(&mut input)?;
	} else {
		std::io::stdin().read_to_string(&mut input)?;
	}

	Ok(input)
}
