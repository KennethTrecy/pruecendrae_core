use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use crate::request::Request;
use crate::response::Response;

mod child;
mod process;

/// Contains functions related to commands
mod command;

#[cfg(test)]
mod fake_process;

/// Represents a task that can be stored and managed.
pub struct Task<'a> {
	name: &'a [u8],
	command: &'a [u8],
	thread: JoinHandle<()>,
	sender: Sender<Request>,
	receiver: Receiver<Response>
}

mod messengers;

use crate::task::command::{run, parse};
use process::Process;

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
		let (program, arguments) = parse(command);

		let thread = thread::spawn(move || {
			let mut process = run(program, arguments);

			for request in receiver.iter() {
				let mut may_continue = true;
				let response;
				match request {
					Request::Output(max_output_size) => {
						let mut output = vec![0; max_output_size];
						let read_size = process.read(&mut output).unwrap();
						let output = (&output[0..read_size]).to_vec();
						response = Response::Output(output);
					},
					Request::Stop => {
						match process.stop() {
							Ok(()) => response = Response::SuccessStop,
							Err(_) => response = Response::FailedStop
						}
					},
					Request::Kill => {
						process.stop().unwrap();
						may_continue = false;
						response = Response::Killed;
					}
				}

				sender.send(response).unwrap();

				if !may_continue { break; }
			}
		});

		return thread;
	}
}
