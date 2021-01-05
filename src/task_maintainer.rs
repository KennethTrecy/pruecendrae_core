use std::collections::HashMap;
use crate::task::Task;

mod request;
mod failed_response;
mod success_response;

/// Maintains a collection of tasks.
pub struct TaskMaintainer<'a> {
	tasks: HashMap<&'a [u8], Task<'a>>
}

mod implementation;

pub use request::Request;
pub use failed_response::FailedResponse;
pub use success_response::SuccessResponse;

impl<'a> TaskMaintainer<'a> {
	pub fn new() -> Self {
		Self { tasks: HashMap::new() }
	}
}
