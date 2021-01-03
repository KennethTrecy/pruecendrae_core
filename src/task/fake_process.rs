pub struct FakeProcess;

use std::io::Result;
use crate::task::process::Process;

impl Process for FakeProcess {
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		let start = 'a' as u8;
		let end = 'z' as u8;

		for (buffer, sample) in buffer.iter_mut().zip(start..end) {
			*buffer = sample;
		}

		Ok(buffer.len())
	}
}
