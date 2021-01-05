use std::ops::Range;
pub const FAKE_SAMPLE: Range<u8> = ('a' as u8)..('z' as u8);

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
		if self.program == "request" {
		let first_argument = self.arguments.pop().unwrap();
			if first_argument == "output_success" {
				for (buffer, sample) in buffer.iter_mut().zip(FAKE_SAMPLE.into_iter().cycle()) {
					*buffer = sample;
				}

				Ok(buffer.len())
			} else if first_argument == "output_failure" {
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
		if self.program == "request" {
			let first_argument = self.arguments.pop().unwrap();
			if first_argument == "success_stop" || first_argument == "success_kill" {
				Ok(())
			} else if first_argument == "error_stop" {
				Err(Error::new(ErrorKind::InvalidInput, "Program was already stopped."))
			} else {
				unimplemented!()
			}
		} else {
			unimplemented!()
		}
	}

	fn check(&mut self) -> bool {
		if self.program == "request" {
			let first_argument = self.arguments.pop().unwrap();
			if first_argument == "currently_running" || first_argument == "currently_stopped" {
				first_argument == "currently_running"
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
	use super::{ErrorKind, FakeProcess, Process, FAKE_SAMPLE};

	#[test]
	fn can_output_with_success() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("output_success")
		]);
		let mut buffer = [0; 20];

		let result = process.read(&mut buffer);

		assert_eq!(buffer.to_vec(), FAKE_SAMPLE.take(20).collect::<Vec<u8>>());
		assert_eq!(result.unwrap(), 20);
	}

	#[test]
	fn cannot_output_with_failure() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("output_failure")
		]);
		let mut buffer = [0; 30];

		let result = process.read(&mut buffer);

		assert_eq!(buffer.to_vec(), [0; 30]);
		assert_eq!(result.unwrap_err().kind(), ErrorKind::BrokenPipe);
	}

	#[test]
	fn can_be_stopped_with_success() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("success_stop")
		]);

		let result = process.stop();

		assert_eq!(result.unwrap(), ());
	}

	#[test]
	fn can_be_stopped_with_error() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("error_stop")
		]);

		let result = process.stop();

		assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
	}

	#[test]
	fn can_be_checked_when_running() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("currently_running")
		]);

		let is_running = process.check();

		assert_eq!(is_running, true);
	}

	#[test]
	fn can_be_checked_when_stopped() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("currently_stopped")
		]);

		let is_running = process.check();

		assert_eq!(is_running, false);
	}
}
