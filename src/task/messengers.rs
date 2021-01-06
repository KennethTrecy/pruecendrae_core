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
	use crate::process::fake_process::{FAKE_OUTPUT_CONTENT, request};
	use super::{Request, Response, Task};

	fn create_task(command: &str) -> Task { Task::new(b"test", command.as_bytes()) }

	#[test]
	pub fn can_request_output_success() {
		let task = create_task(request::OUTPUT_SUCCESS);
		let max_output_size = 10;
		let expected_content = FAKE_OUTPUT_CONTENT.take(max_output_size).collect();
		let expected_reponse = Response::Output(Ok(expected_content));

		task.send_request(Request::Output(max_output_size)).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_output_error() {
		let task = create_task(request::OUTPUT_FAILURE);
		let max_output_size = 15;
		let expected_reponse = Response::Output(Err(()));

		task.send_request(Request::Output(max_output_size)).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_success_stop() {
		let task = create_task(request::STOP_SUCCESS);
		let expected_reponse = Response::Stop(Ok(()));

		task.send_request(Request::Stop).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_error_stop() {
		let task = create_task(request::STOP_FAILURE);
		let expected_reponse = Response::Stop(Err(()));

		task.send_request(Request::Stop).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_success_kill() {
		let task = create_task(request::KILL_SUCCESS);
		let expected_reponse = Response::Kill(Ok(()));

		task.send_request(Request::Kill).unwrap();
		let response = task.receive_response();

		assert_eq!(response, expected_reponse);
	}
}
