use std::process::{Child, Command, Stdio};

pub fn run_command(program: String, arguments: Vec<String>) -> Child {
	Command::new(program)
		.args(&arguments)
		.stdout(Stdio::piped())
		.stdin(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.unwrap()
}
