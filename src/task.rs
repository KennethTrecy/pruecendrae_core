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
pub struct Task<'a> {
	name: &'a [u8],
	command: &'a [u8],
	thread: JoinHandle<()>,
	sender: Sender<Request>,
	receiver: Receiver<Response>
}

mod messengers;

use crate::task::command::{create_thread, parse};

impl<'a> Task<'a> {
	/// Creates a Task and runs the command.
	pub fn new(name: &'a [u8], command: &'a [u8]) -> Self {
		let (thread, sender, receiver) = Self::run_command(command);
		Self {
			name,
			command,
			thread,
			sender,
			receiver
		}
	}

	fn run_command(command: &'a [u8]) -> (JoinHandle<()>, Sender<Request>, Receiver<Response>) {
		let (program, arguments) = parse(command);
		let thread = create_thread(program, arguments);
		thread
	}
}
