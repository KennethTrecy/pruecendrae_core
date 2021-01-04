use std::io::Result;

/// Interface for different types of process.
pub trait Process {
	/// Reads the output of the process
	fn read(&mut self, _: &mut [u8]) -> Result<usize>;

	/// Stops the process.
	fn stop(&mut self) -> Result<()>;

	/// Checks if the process is still running.
	fn check(&mut self) -> bool;
}
