use std::env;
use std::process::{Command, ExitCode};
use std::str::FromStr;

use env_logger::{Builder, Env};

enum Subcommand {
	Format,
	Lint,
	Test,
	Mutants,
}

fn main() -> ExitCode {
	Builder::from_env(Env::default().default_filter_or("info"))
		.format_timestamp(None)
		.init();

	let subcommand =
		match Subcommand::try_from(env::args().collect::<Vec<_>>().get(1)) {
			Ok(cmd) => cmd,
			Err(e) => {
				log::error!("{e}");
				return ExitCode::FAILURE;
			}
		};

	let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
	let result = match subcommand {
		Subcommand::Format => run_format(),
		Subcommand::Lint => run_lint(&cargo),
		Subcommand::Test => run_tests(&cargo),
		Subcommand::Mutants => run_mutants(&cargo),
	};

	match result {
		Ok(()) => ExitCode::SUCCESS,
		Err(e) => {
			log::error!("Task failed: {e}");
			ExitCode::FAILURE
		}
	}
}

impl FromStr for Subcommand {
	type Err = &'static str;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"format" => Ok(Self::Format),
			"lint" => Ok(Self::Lint),
			"test" => Ok(Self::Test),
			"mutants" => Ok(Self::Mutants),
			_ => Err("Expected 'format' | 'lint' | 'test' | 'mutants'."),
		}
	}
}

impl TryFrom<Option<&String>> for Subcommand {
	type Error = &'static str;

	fn try_from(value: Option<&String>) -> Result<Self, Self::Error> {
		value.map_or(Err("Expected a subcommand"), |s| s.parse())
	}
}

fn run_format() -> Result<(), String> {
	// rustup needed for nightly toolchain via xtask
	let status = Command::new("rustup")
		.args(["run", "nightly", "cargo", "fmt", "--all"])
		.status()
		.map_err(|e| format!("Failed to spawn fmt: {e}"))?;

	if status.success() { Ok(()) } else { Err("Format failed.".to_string()) }
}

fn run_lint(cargo: &str) -> Result<(), String> {
	let clippy_status = Command::new(cargo)
		.args([
			"clippy",
			"--workspace",
			"--all-targets",
			"--all-features",
			"--",
			"-Dwarnings",
			"-Dclippy::all",
		])
		.status()
		.map_err(|e| format!("Failed to spawn clippy: {e}"))?;

	// rustup needed for nightly toolchain via xtask
	let fmt_status = Command::new("rustup")
		.args(["run", "nightly", "cargo", "fmt", "--all", "--check"])
		.status()
		.map_err(|e| format!("Failed to spawn fmt: {e}"))?;

	match (fmt_status.success(), clippy_status.success()) {
		(true, true) => Ok(()),
		(false, _) => Err("Code formatting issues found.".to_string()),
		(_, false) => Err("Clippy linting issues found.".to_string()),
	}
}

fn run_tests(cargo: &str) -> Result<(), String> {
	let status = Command::new(cargo)
		.args([
			"llvm-cov",
			"nextest",
			"--workspace",
			"--all-targets",
			"--all-features",
			"--no-fail-fast",
		])
		.status()
		.map_err(|e| format!("Failed to spawn tests: {e}"))?;

	if status.success() {
		Ok(())
	} else {
		Err("One or more tests failed.".to_string())
	}
}

fn run_mutants(cargo: &str) -> Result<(), String> {
	let status = Command::new(cargo)
		.args(["mutants", "--in-place", "--test-tool=nextest"])
		.status()
		.map_err(|e| format!("Failed to spawn cargo mutants: {e}"))?;

	if !status.success() {
		log::warn!("Mutation tests identified gaps or failed to execute.");
	}

	Ok(())
}
