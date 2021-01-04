use std::collections::HashMap;
use crate::task::Task;

/// Maintains a collection of tasks.
pub struct TaskMaintainer<'a> {
	tasks: HashMap<&'a [u8], Task<'a>>
}

impl<'a> TaskMaintainer<'a> {
	pub fn new() -> Self {
		Self { tasks: HashMap::new() }
	}
}
