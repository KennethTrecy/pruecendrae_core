use std::sync::mpsc::SendError;
use crate::task::request::Request;
use crate::task::response::Response;
use super::Task;

impl<'a> Task<'a> {
	pub fn send_request(&self, request: Request) -> Result<(), SendError<Request>> {
		self.sender.send(request)
	}

	pub fn receive_response(&self) -> Response {
		self.receiver.recv().unwrap()
	}
}

#[cfg(test)]
mod t {
	use crate::task::fake_process::SAMPLE;
	use super::{Request, Response, Task};

	#[test]
	pub fn can_request_output() {
		let task = Task::new(b"test", b"request output");
		let max_output_size = 10;
		let expected_response_content = SAMPLE.into_iter().cycle().take(max_output_size).collect();
		let expected_reponse = Response::Output(expected_response_content);

		task.send_request(Request::Output(max_output_size)).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_success_stop() {
		let task = Task::new(b"test", b"request success_stop");
		let expected_reponse = Response::SuccessStop;

		task.send_request(Request::Stop).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_error_stop() {
		let task = Task::new(b"test", b"request error_stop");
		let expected_reponse = Response::FailedStop;

		task.send_request(Request::Stop).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_success_kill() {
		let task = Task::new(b"test", b"request success_kill");
		let expected_reponse = Response::Killed;

		task.send_request(Request::Kill).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}
}