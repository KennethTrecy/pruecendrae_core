use std::collections::HashMap;
use crate::task::Task;

#[macro_use]
mod macros;
mod request;
mod response;

/// Maintains a collection of tasks.
pub struct TaskMaintainer<'a> {
	tasks: HashMap<&'a [u8], Task<'a>>
}

mod implementation;

pub use request::Request;
pub use response::Response;

impl<'a> TaskMaintainer<'a> {
	pub fn new() -> Self {
		Self { tasks: HashMap::new() }
	}
}
