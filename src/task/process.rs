use std::io::Result;

/// Interface for different types of process.
pub trait Process {
	/// Reads the output of the process
	fn read(&mut self, _: &mut [u8]) -> Result<usize>;
}
