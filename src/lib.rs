use std::thread::{self, JoinHandle};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Sender, Receiver};

mod task_message;

pub use task_message::TaskMessage;

pub struct Task<'a> {
	name: &'a [u8],
	command: &'a [u8],
	thread: JoinHandle<()>,
	sender: Sender<TaskMessage>,
	receiver: Receiver<TaskMessage>
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
		sender: Sender<TaskMessage>,
		receiver: Receiver<TaskMessage>
	) -> JoinHandle<()> {
		let (program, arguments) = parse_command(command);

		let thread = thread::spawn(move || {
			let mut command = Command::new(program)
				.args(&arguments)
				.stdout(Stdio::piped())
				.stdin(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.unwrap();

			for request in receiver.iter() {
				let response;
				match request {
					TaskMessage::RequestOutput => {
						let mut output = [0; 80];
						if let Some(mut child) = command.stdout {
							use std::io::Read;
							child.read(&mut output).unwrap();
							command.stdout = Some(child);
						}

						response = TaskMessage::ResponseOutput(output.to_vec());
					},
					_ => response = TaskMessage::ResponseOutput(Vec::new())
				}

				sender.send(response).unwrap();
			}
		});

		return thread;
	}
}

fn parse_command(command: &[u8]) -> (String, Vec<String>) {
	let mut program = None;
	let mut arguments = Vec::new();
	let mut start = 0;
	let mut end = 0;
	let command_length = command.len();

	while end <= command_length {
		if end == command_length && command[end] == ' ' as u8 {
			let fragment = &command[start..end];
			let fragment = String::from_utf8_lossy(&fragment[..]).into_owned();

			match program {
				Some(_) => arguments.push(fragment),
				None => program = Some(fragment)
			}

			end += 1;
			start = end;
		}

		end += 1;
	}

	(program.unwrap(), arguments)
}
