#[cfg(not(test))]
use std::process::{Command, Stdio};

use crate::process::Process;

#[cfg(test)]
use crate::process::fake_process::FakeProcess;

pub fn run(program: String, arguments: Vec<String>) -> impl Process {
	#[cfg(not(test))]
	{
		Command::new(program)
			.args(&arguments)
			.stdout(Stdio::piped())
			.stdin(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.unwrap()
	}

	#[cfg(test)]
	{
		FakeProcess::new(program, arguments)
	}
}
