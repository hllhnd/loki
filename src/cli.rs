use std::env::current_dir;

use color_eyre::Report;
use indoc::indoc;

use crate::build_project;
use crate::config::Package;
use crate::config::PackageKind;
use crate::config::Standard;

/// Process the command line arguments and run the appropriate subcommand, assuming command arguments start at index 1.
pub fn process(args: &[String]) -> Result<(), Report> {
	match args.get(1).map(|f| f.as_str()) {
		Some("build") => subcommand_build(),
		Some("new") => subcommand_new(args.get(2)),
		Some("-v" | "--version") => subcommand_version(),
		Some("-h" | "--help") | None => subcommand_help(),
		_ => Ok(println!("Unknown command/flag '{}'. See '--help' for usage.", args[1])),
	}
}

fn subcommand_build() -> Result<(), Report> {
	build_project()
}

fn subcommand_new(name: Option<&String>) -> Result<(), Report> {
	match name {
		Some(name) => {
			let package = Package {
				name:     name.clone(),
				kind:     PackageKind::Application,
				version:  "0.1.0".to_string(),
				standard: Standard::C99,
			};

			let project_directory = current_dir()?;
			let loki_toml = project_directory.join("loki.toml");
			let serialized = package.serialize();

			std::fs::write(loki_toml, serialized)?;

			Ok(())
		},
		None => {
			print!(indoc! {"
				error: missing argument 'name'

				Usage: loki new <name>
			"});
			Ok(())
		},
	}
}

fn subcommand_version() -> Result<(), Report> {
	print!(indoc! {"
		The Loki Build System, version 0.0.3

		Copyright (c) 2024 Reperak

		Loki is free software licensed under the GNU GPL version 3 or later.

		If you did not receive a copy of the license with this program, you may obtain
		one at <http://gnu.org/licenses/gpl.html>.
	"});

	Ok(())
}

fn subcommand_help() -> Result<(), Report> {
	print!(indoc! {"
		The Loki Build System, version 0.0.3

		Copyright (c) 2024 Reperak

		Subcommands:
		        build           Build a Loki project
		        new             Create a new Loki project

		Usage:
		        --help          Show this text and exit
		        --version       Show version information
	"});

	Ok(())
}
