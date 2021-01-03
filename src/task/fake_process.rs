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

	fn terminate(&mut self) -> Result<()> {
		if self.program == "request" {
			let first_argument = self.arguments.pop().unwrap();
			if first_argument == "success_termination" {
				Ok(())
			} else if first_argument == "error_termination" {
				Err(Error::new(ErrorKind::InvalidInput, "Program was already terminated."))
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
	fn can_be_terminated_with_success() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("success_termination")
		]);

		let result = process.terminate();

		assert_eq!(result.unwrap(), ());
	}

	#[test]
	fn can_be_terminated_with_error() {
		let mut process = FakeProcess::new(String::from("request"), vec![
			String::from("error_termination")
		]);

		let result = process.terminate();

		assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
	}
}
