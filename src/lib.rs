use std::thread::{self, JoinHandle};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Sender, Receiver};

pub struct Task<'a> {
	name: &'a [u8],
	command: &'a [u8],
	thread: JoinHandle<()>,
	sender: Sender<Vec<u8>>,
	receiver: Receiver<Vec<u8>>
}

impl<'a> Task<'a> {
	pub fn new(name: &'a [u8], command: &'a [u8]) -> Self {
		let (exsender, inreceiver) = mpsc::channel();
		let (insender, exreceiver) = mpsc::channel();
		let thread = Self::run_command(command, insender, inreceiver);
		Self {
			name,
			command,
			thread,
			sender: exsender,
			receiver: exreceiver
		}
	}

	fn run_command(
		command: &'a [u8],
		sender: Sender<Vec<u8>>,
		receiver: Receiver<Vec<u8>>
	) -> JoinHandle<()> {
		let mut command_to_execute = None;
		let mut command_arguments = Vec::new();
		let mut start = 0;
		let mut end = 0;
		let command_length = command.len();

		while end <= command_length {
			if end == command_length && command[end] == ' ' as u8 {
				let fragment = &command[start..end];
				let fragment = String::from_utf8_lossy(&fragment[..]).into_owned();

				match command_to_execute {
					Some(_) => command_arguments.push(fragment),
					None => command_to_execute = Some(fragment)
				}

				end += 1;
				start = end;
			}

			end += 1;
		}

		let thread = thread::spawn(move || {
			let command_to_execute = command_to_execute.take().unwrap();
			let mut command = Command::new(command_to_execute)
				.args(&command_arguments)
				.stdout(Stdio::piped())
				.stdin(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.unwrap();

			for _ in receiver.iter() {
				let mut output = [0; 80];
				if let Some(mut child) = command.stdout {
					use std::io::Read;
					child.read(&mut output).unwrap();
					command.stdout = Some(child);
				}

				sender.send(output.to_vec()).unwrap();
			}
		});

		return thread;
	}
}
