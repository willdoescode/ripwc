use std::path::PathBuf;
use clap::{Clap, AppSettings};

#[derive(Clap, Debug)]
#[clap(author = "William L. <williamlane923@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Input file
	#[clap(parse(from_os_str))]
	pub file: Option<PathBuf>,

	/// Show line count
	#[clap(short)]
	pub l: bool,

	/// Show number of words
	#[clap(short)]
	pub w: bool,

	/// Show number of bytes
	#[clap(short)]
	pub c: bool,

	/// Show number of characters
	#[clap(short)]
	pub m: bool,
}
