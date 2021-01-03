use std::io::Result;

/// Interface for different types of process.
pub trait Process {
	/// Reads the output of the process
	fn read(&self, buffer: &mut [u8]) -> Result<usize>;
}
