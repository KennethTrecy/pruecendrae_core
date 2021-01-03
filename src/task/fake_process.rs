/// Represents the fake process
pub struct FakeProcess;

use std::ops::Range;
pub const SAMPLE: Range<u8> = ('a' as u8)..('z' as u8);

use std::io::Result;
use crate::task::process::Process;

impl Process for FakeProcess {
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		for (buffer, sample) in buffer.iter_mut().zip(SAMPLE) {
			*buffer = sample;
		}

		Ok(buffer.len())
	}
}

#[cfg(test)]
mod t {
	use super::{FakeProcess, Process, SAMPLE};

	#[test]
	fn can_read() {
		let mut process = FakeProcess;
		let mut buffer = [0; 20];

		let result = process.read(&mut buffer);

		assert_eq!(buffer.to_vec(), (SAMPLE).take(20).collect::<Vec<u8>>());
		assert_eq!(result.unwrap(), 20);
	}
}
