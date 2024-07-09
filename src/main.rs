#![allow(clippy::redundant_field_names)]

mod cli;
mod compiler;
mod config;
mod directory;
mod executable;
mod node;

use std::cell::RefCell;
use std::env::args;
use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;

use color_eyre::Report;
use compiler::CSourceToObject;
use compiler::LinkObjectsToBinary;
use config::Package;
use directory::CreateDirectory;
use itertools::Itertools;
use node::Node;
use walkdir::WalkDir;

fn main() -> Result<(), Report> {
	color_eyre::install()?;

	let args = args().collect::<Vec<_>>();

	cli::process(&args)?;

	Ok(())
}

fn build_project() -> Result<(), Report> {
	let (loki_toml, source_directory, target_directory, object_directory) = current_dir()?
		.ancestors()
		.map(PathBuf::from)
		.map(|project_directory| {
			(
				project_directory.join("loki.toml"),
				project_directory.join("src"),
				project_directory.join("target"),
				project_directory.join("target/obj"),
			)
		})
		.filter(|(loki_toml, ..)| loki_toml.exists())
		.last()
		.ok_or(io::Error::new(
			io::ErrorKind::NotFound,
			"loki project directory not found",
		))?;

	let package: Package =
		Package::parse(&fs::read_to_string(loki_toml)?).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

	let source_files = WalkDir::new(source_directory)
		.into_iter()
		.map(|dir| dir.unwrap().into_path())
		.filter(|path| path.extension().is_some_and(|d| d == "c"))
		.collect_vec();

	let create_target_directory_node = Rc::new(RefCell::new(Node {
		executable: Box::new(CreateDirectory {
			directory: target_directory.clone(),
		}),
		children:   Vec::new(),
	}));

	let create_object_directory_node = Rc::new(RefCell::new(Node {
		executable: Box::new(CreateDirectory {
			directory: object_directory.clone(),
		}),
		children:   Vec::new(),
	}));

	let c2so_nodes = source_files
		.clone()
		.into_iter()
		.map(|source| {
			let cs2o = CSourceToObject {
				input:            source,
				object_directory: object_directory.clone(),
			};

			let node = Node {
				executable: Box::new(cs2o),
				children:   vec![
					Rc::clone(&create_target_directory_node),
					Rc::clone(&create_object_directory_node),
				],
			};

			Rc::new(RefCell::new(node))
		})
		.collect_vec();

	let lo2b_node = Rc::new(RefCell::new(Node {
		executable: Box::new(LinkObjectsToBinary {
			inputs: source_files,
			output: target_directory.join(&package.name),
		}),
		children:   [
			&c2so_nodes[..],
			&[
				Rc::clone(&create_target_directory_node),
				Rc::clone(&create_target_directory_node),
			],
		]
		.concat(),
	}));

	execute_node(lo2b_node).unwrap();

	Ok(())
}

fn execute_node(node: Rc<RefCell<Node>>) -> Result<i32, Box<dyn Error + Send + Sync>> {
	for child in &node.borrow().children {
		execute_node(Rc::clone(child))?;
	}

	node.borrow_mut().executable.execute()
}
