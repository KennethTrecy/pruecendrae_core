use crate::request::Request;
use crate::response::Response;
use super::Task;

impl<'a> Task<'a> {
	pub fn request(&self, request: Request) -> Response {
		self.sender.send(request).unwrap();
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

		let response = task.request(Request::Output(max_output_size));

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_success_termination() {
		let task = Task::new(b"test", b"request success_termination");
		let expected_reponse = Response::SuccessTermination;

		let response = task.request(Request::Terminate);

		assert_eq!(response, expected_reponse);
	}

	#[test]
	pub fn can_request_error_termination() {
		let task = Task::new(b"test", b"request error_termination");
		let expected_reponse = Response::ErrorTermination;

		let response = task.request(Request::Terminate);

		assert_eq!(response, expected_reponse);
	}
}
