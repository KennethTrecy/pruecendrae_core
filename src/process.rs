use std::io::Result;

/// Interface for different types of process.
pub trait Process {
	/// Reads the output of the process.
	fn read(&mut self, _: &mut [u8]) -> Result<usize>;

	/// Stops the process.
	fn stop(&mut self) -> Result<()>;

	/// Checks if the process is still running.
	fn check(&mut self) -> bool;
}

/// Contains the implementation of `Process` for `std::process::Child`.
mod child;

/// Contains the `FakeProcess` that is usually used as stub.
#[cfg(test)]
pub mod fake_process;
