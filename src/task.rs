use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use crate::request::Request;
use crate::response::Response;

mod process;
mod run_command;
mod parse_command;

/// Represents a task that can be stored and managed.
pub struct Task<'a> {
	name: &'a [u8],
	command: &'a [u8],
	thread: JoinHandle<()>,
	sender: Sender<Request>,
	receiver: Receiver<Response>
}

use run_command::run_command;
use parse_command::parse_command;

impl<'a> Task<'a> {
	/// Creates a Task and runs the command.
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

	fn run_command(command: &'a [u8], sender: Sender<Response>, receiver: Receiver<Request>)
	-> JoinHandle<()> {
		let (program, arguments) = parse_command(command);

		let thread = thread::spawn(move || {
			let mut command = run_command(program, arguments);

			for request in receiver.iter() {
				let response;
				match request {
					Request::Output => {
						let mut output = [0; 80];
						if let Some(mut child) = command.stdout {
							use std::io::Read;
							child.read(&mut output).unwrap();
							command.stdout = Some(child);
						}

						response = Response::Output(output.to_vec());
					}
				}

				sender.send(response).unwrap();
			}
		});

		return thread;
	}
}
