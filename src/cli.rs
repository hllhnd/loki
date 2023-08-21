use color_eyre::Report;
use crate::build_project;
use crate::config::Project;
use std::env::current_dir;

pub fn process(args: &[String]) -> Result<(), Report> {
    match args.get(1).map(|f| f.as_str()) {
        Some("build") => {
            build_project()?;
        },

        Some("-v" | "--version") => {
            #[rustfmt::skip]
            println!(
                "The Loki Build System, version 0.1.0\n\
                \n\
                Copyright (c) 2023 Reperak\n\
                \n\
                Loki is free software licensed under the GNU GPL version 3 or later.\n\
                \n\
                If you did not receive a copy of the license with this program, you may obtain\n\
                one at <http://gnu.org/licenses/gpl.html>."
            );
        },

        Some("-h" | "--help") | None => {
            #[rustfmt::skip]
            println!(
                "The Loki Build System\n\
                \n\
                Copyright (c) 2023 Reperak\n\
                \n\
                Subcommands:\n    \
                    build           Build a Loki project\n    \
                    new             Create a new Loki project\n\
                \n\
                Usage:\n    \
                    --help          Show this text and exit\n    \
                    --version       Show version information"
            );
        },

        Some("new") =>
            if let Some(name) = args.get(2) {
                // TODO: allow for creating projects in other directories
                Project::with_name(name).generate_at(current_dir()?)?;
            } else {
                #[rustfmt::skip]
                println!(
                    "error: missing argument 'path'\n\
                    \n\
                    Usage: loki new <path>"
                );
            },

        _ => {
            println!("Unknown command/flag '{}'. See '--help' for usage.", args[1]);
        },
    };

    Ok(())
}
