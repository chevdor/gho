use clap::{crate_authors, crate_version, AppSettings, Clap};

/// Command line utility for the tera templating engine. You need to provide a template using the tera syntax
/// as well as some data (various format are supported).
#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Name of a remote
	#[clap(index = 1)]
	pub remote: Option<String>,
}
