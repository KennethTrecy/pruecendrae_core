mod result;

/// Contains functions related to commands
mod command;

/// Contains variants of `Request` messages.
pub mod request;

/// Contains variants of `Response` messages.
pub mod response;

use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
pub use request::Request;
pub use response::Response;

/// Represents a task that can be stored and managed.
pub struct Task {
	command: String,
	thread: JoinHandle<()>,
	sender: Sender<Request>,
	receiver: Receiver<Response>
}

mod messengers;
mod join;

use crate::task::command::{create_thread, parse};

impl Task {
	/// Creates a Task and runs the command.
	pub fn new(command: &[u8]) -> Self {
		let (thread, sender, receiver) = Self::run_command(command);
		let command = String::from_utf8(command.to_vec()).unwrap();
		Self {
			command,
			thread,
			sender,
			receiver
		}
	}

	pub fn command(&self) -> &str { &self.command }

	fn run_command(command: &[u8]) -> (JoinHandle<()>, Sender<Request>, Receiver<Response>) {
		let (program, arguments) = parse(command);
		let thread = create_thread(program, arguments);
		thread
	}
}
