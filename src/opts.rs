use clap::{crate_authors, crate_version, Parser};

/// Command line utility to quickly open your github/gitlab repo
/// in a browser.
#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	/// Name of a remote
	#[clap(index = 1)]
	pub remote: Option<String>,
}
