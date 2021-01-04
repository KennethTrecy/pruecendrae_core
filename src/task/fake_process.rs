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

use std::ops::Range;
pub const SAMPLE: Range<u8> = ('a' as u8)..('z' as u8);

use std::io::{Error, ErrorKind, Result};
use crate::task::process::Process;

impl Process for FakeProcess {
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		for (buffer, sample) in buffer.iter_mut().zip(SAMPLE.into_iter().cycle()) {
			*buffer = sample;
		}

		Ok(buffer.len())
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
	use super::{ErrorKind, FakeProcess, Process, SAMPLE};

	#[test]
	fn can_read() {
		let mut process = FakeProcess::new(String::new(), vec![String::new()]);
		let mut buffer = [0; 20];

		let result = process.read(&mut buffer);

		assert_eq!(buffer.to_vec(), (SAMPLE).take(20).collect::<Vec<u8>>());
		assert_eq!(result.unwrap(), 20);
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
