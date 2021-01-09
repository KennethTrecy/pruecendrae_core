use std::collections::HashMap;
use crate::task::Task;

#[macro_use]
mod macros;
mod request;
mod response;

/// Maintains a collection of tasks.
pub struct TaskMaintainer {
	tasks: HashMap<String, Task>
}

mod implementation;

pub use request::Request;
pub use response::Response;

impl TaskMaintainer {
	pub fn new() -> Self {
		Self { tasks: HashMap::new() }
	}
}
