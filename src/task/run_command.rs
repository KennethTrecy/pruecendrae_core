// When being tested, the imports at line 4, and parameters at line 5, are being marked as unused.
// Using the attribute below, it will remove the warnings.
#![allow(unused_imports, unused_variables)]

use std::process::{Child, Command, Stdio};
use crate::task::process::Process;

#[cfg(test)]
use crate::task::fake_process::FakeProcess;

pub fn run_command(program: String, arguments: Vec<String>) -> impl Process {
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
