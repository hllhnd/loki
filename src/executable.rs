use std::error::Error;
use std::process::Command;

pub trait Executable {
	fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>>;
}

impl Executable for Command {
	fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>> {
		self.spawn()?
			.wait()?
			.code()
			.ok_or_else(|| todo!("child process signal handling"))
	}
}

impl Executable for () {
	fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>> {
		Ok(0)
	}
}
