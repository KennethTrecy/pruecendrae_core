use std::ops::Range;
pub const FAKE_OUTPUT_CONTENT: Range<u8> = ('a' as u8)..('z' as u8);

pub mod request {
	pub const PROGRAM: &'static str = "request";
	pub const START_SUCCESS: &'static str = "request start_success";
	pub const START_FAILURE: &'static str = "request start_failure";
	pub const STOP_SUCCESS: &'static str = "request stop_success";
	pub const STOP_FAILURE: &'static str = "request stop_failure";
	pub const CHECK_SUCCESS: &'static str = "request check_success";
	pub const CHECK_FAILURE: &'static str = "request check_failure";
	pub const OUTPUT_SUCCESS: &'static str = "request output_success";
	pub const OUTPUT_FAILURE: &'static str = "request output_failure";
	pub const KILL_SUCCESS: &'static str = "request kill_success";
	pub const KILL_FAILURE: &'static str = "request kill_failure";
}

/// Represents the fake process
pub struct FakeProcess {
	program: String,
	arguments: Vec<String>
}

impl FakeProcess {
	pub fn new(program: String, arguments: Vec<String>) -> Self {
		Self { program, arguments }
	}
}

use std::io::{Error, ErrorKind, Result};
use super::Process;

impl Process for FakeProcess {
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		if self.program == request::PROGRAM {
		let first_argument = self.arguments.pop().unwrap();
			if request::OUTPUT_SUCCESS.ends_with(&first_argument) {
				for (buffer, sample) in buffer.iter_mut().zip(FAKE_OUTPUT_CONTENT.into_iter().cycle()) {
					*buffer = sample;
				}

				Ok(buffer.len())
			} else if request::OUTPUT_FAILURE.ends_with(&first_argument) {
				Err(Error::new(
					ErrorKind::BrokenPipe,
					"There is a problem in reading the output of the program."))
			} else {
				unimplemented!()
			}
		} else {
			unimplemented!()
		}
	}

	fn stop(&mut self) -> Result<()> {
		if self.program == request::PROGRAM {
			let first_argument = self.arguments.pop().unwrap();
			if request::STOP_SUCCESS.ends_with(&first_argument)
			|| request::KILL_SUCCESS.ends_with(&first_argument) {
				Ok(())
			} else if request::STOP_FAILURE.ends_with(&first_argument) {
				Err(Error::new(ErrorKind::InvalidInput, "Program was already stopped."))
			} else {
				unimplemented!()
			}
		} else {
			unimplemented!()
		}
	}

	fn check(&mut self) -> bool {
		if self.program == request::PROGRAM {
			let first_argument = self.arguments.pop().unwrap();
			if request::CHECK_SUCCESS.ends_with(&first_argument)
			|| request::CHECK_FAILURE.ends_with(&first_argument) {
				request::CHECK_SUCCESS.ends_with(&first_argument)
			} else {
				unimplemented!()
			}
		} else {
			unimplemented!()
		}
	}
}

#[cfg(test)]
mod t {
	use super::{ErrorKind, FAKE_OUTPUT_CONTENT, FakeProcess, Process, request};

	fn create_fake_process(command: &str) -> FakeProcess {
		let program = request::PROGRAM;
		let argument = &command[program.len() + 1..];
		FakeProcess::new(String::from(program), vec![String::from(argument)])
	}

	#[test]
	fn can_output_with_success() {
		let mut process = create_fake_process(request::OUTPUT_SUCCESS);
		let mut buffer = [0; 20];

		let result = process.read(&mut buffer);

		assert_eq!(buffer.to_vec(), FAKE_OUTPUT_CONTENT.take(20).collect::<Vec<u8>>());
		assert_eq!(result.unwrap(), 20);
	}

	#[test]
	fn cannot_output_with_failure() {
		let mut process = create_fake_process(request::OUTPUT_FAILURE);
		let mut buffer = [0; 30];

		let result = process.read(&mut buffer);

		assert_eq!(buffer.to_vec(), [0; 30]);
		assert_eq!(result.unwrap_err().kind(), ErrorKind::BrokenPipe);
	}

	#[test]
	fn can_be_stopped_with_success() {
		let mut process = create_fake_process(request::STOP_SUCCESS);

		let result = process.stop();

		assert_eq!(result.unwrap(), ());
	}

	#[test]
	fn can_be_stopped_with_error() {
		let mut process = create_fake_process(request::STOP_FAILURE);

		let result = process.stop();

		assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
	}

	#[test]
	fn can_be_checked_when_running() {
		let mut process = create_fake_process(request::CHECK_SUCCESS);

		let is_running = process.check();

		assert_eq!(is_running, true);
	}

	#[test]
	fn can_be_checked_when_stopped() {
		let mut process = create_fake_process(request::CHECK_FAILURE);

		let is_running = process.check();

		assert_eq!(is_running, false);
	}
}
