use std::env::current_dir;

use color_eyre::Report;
use indoc::indoc;

use crate::build_project;
use crate::config::Project;

pub fn process(args: &[String]) -> Result<(), Report> {
    match args.get(1).map(|f| f.as_str()) {
        Some("build") => {
            build_project()?;
        },

        Some("-v" | "--version") => {
            print!(indoc! {"
                The Loki Build System, version 0.1.0

                Copyright (c) 2023 Reperak

                Loki is free software licensed under the GNU GPL version 3 or later.

                If you did not receive a copy of the license with this program, you may obtain
                one at <http://gnu.org/licenses/gpl.html>.
            "});
        },

        Some("-h" | "--help") | None => {
            print!(indoc! {"
                The Loki Build System

                Copyright (c) 2023 Reperak

                Subcommands:
                    build           Build a Loki project
                    new             Create a new Loki project

                Usage:
                    --help          Show this text and exit
                    --version       Show version information
            "});
        },

        Some("new") =>
            if let Some(name) = args.get(2) {
                // TODO: allow for creating projects in other directories
                Project::with_name(name).generate_at(current_dir()?)?;
            } else {
                print!(indoc! {"
                    error: missing argument 'name'

                    Usage: loki new <name>
                "});
            },

        _ => {
            println!("Unknown command/flag '{}'. See '--help' for usage.", args[1]);
        },
    };

    Ok(())
}
