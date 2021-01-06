use std::thread::Result;
use super::Task;

impl<'a> Task<'a> {
	/// Joins the thread of the task. However, a kill request must be sent first.
	pub fn join(self) -> Result<()> {
		self.thread.join()
	}
}

#[cfg(test)]
mod t {
	use crate::task::{Request, Response};
	use crate::process::fake_process::request;
	use super::Task;

	#[test]
	pub fn can_join_after_kill() {
		let task = Task::new(b"test", request::KILL_SUCCESS.as_bytes());
		let expected_reponse = Response::Killed(Ok(()));

		task.send_request(Request::Kill).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
		assert_eq!(task.join().unwrap(), ());
	}
}
