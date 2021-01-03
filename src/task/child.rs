use std::io::{Read, Result};
use std::process::Child;
use crate::task::process::Process;

impl Process for Child {
	fn read(&mut self, mut buffer: &mut [u8]) -> Result<usize> {
		let mut stdout = self.stdout.take().unwrap();
		let read_result = stdout.read(&mut buffer);
		self.stdout = Some(stdout);
		read_result
	}
}
