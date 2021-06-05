mod opts;
use std::process::Command;
use std::str;

use clap::{crate_name, crate_version, Clap};
use env_logger::Env;
use log::{debug, info};
use opts::*;
use regex::Regex;

fn run_command(cmd: &str) -> Result<String, String> {
	let output = if cfg!(target_os = "windows") {
		Command::new("cmd").args(&["/C", cmd]).output().expect("failed to call git")
	} else {
		Command::new("sh").arg("-c").arg(cmd).output().expect("failed to call git")
	};

	if output.status.success() {
		Ok(str::from_utf8(&output.stdout).unwrap().to_string())
	} else {
		Err(str::from_utf8(&output.stderr).unwrap().to_string())
	}
}

fn get_url(remote: &str) -> Result<String, String> {
	debug!("get url for: {}", remote);

	let cmd = format!("git config remote.{}.url", remote);
	debug!("{}", cmd);
	let url = run_command(&cmd)?;
	debug!("url: {}", url);

	let re = Regex::new(r"git@(.*):(.*).git.*").unwrap();
	let caps = re.captures(&url).unwrap();

	let site = caps.get(1).map_or("", |m| m.as_str());
	let repo = caps.get(2).map_or("", |m| m.as_str());

	debug!("site: {}", site);
	debug!("repo: {}", repo);

	Ok(format!("https://{site}/{repo}", site = site, repo = repo))
}

fn main() -> Result<(), String> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	info!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();
	let git_remote = run_command("git remote").unwrap();
	debug!("git remote: {:?}", git_remote);

	let remote = match opts.remote {
		Some(ref r) => r,
		_ => &git_remote,
	};
	let remote = String::from(remote);
	let remote = remote.trim_end();

	let url = get_url(remote).unwrap_or_else(|_| panic!("Failed getting url for '{}'", remote));

	debug!("Opening url: {}", url);
	print!("{}", url);
	match webbrowser::open(&url) {
		Ok(_) => Ok(()),
		_ => Err("Problem while opening browser".to_string()),
	}
}
