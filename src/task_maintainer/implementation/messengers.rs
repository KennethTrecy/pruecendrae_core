use crate::{Request as MaintainerRequest, Response as MaintainerResponse};
use crate::task::{Request as TaskRequest, Response as TaskResponse};
use super::TaskMaintainer;

impl<'a> TaskMaintainer<'a> {
	pub fn send_request(&self, request_type: MaintainerRequest<'a>) {
		macro_rules! request {
			(for each $names:ident, $name:ident $(with $($arguments:ident)+)?) => {
				{
					for name in $names {
						if self.tasks.contains_key(name) {
							let task = self.tasks.get(name).unwrap();
							task.send_request(TaskRequest::$name$(($($arguments,)*))?).unwrap();
						} else {
							todo!()
						}
					}
				}
			};
		}

		match request_type {
			MaintainerRequest::Output(max_output_size, names) => request!{
				for each names, Output with max_output_size
			},
			MaintainerRequest::Check(names) => request!{for each names, Check},
			MaintainerRequest::Stop(names) => request!{for each names, Stop},
			_ => { todo!() }
		}
	}

	pub fn receive_response(&self, names: Vec<&'a [u8]>) -> MaintainerResponse<'a> {
		let mut response = self.receive_initial_response(&names);

		macro_rules! receive_other {
			(
				$response_name:ident $(with $response:ident)?
				that will be classified as either one of the $successes:ident or $failures:ident
			) => {
				{
					let mut has_skipped_initial_output = false;
					for name in names {
						if let Some(task) = self.tasks.get(name) {
							if has_skipped_initial_output {
								if let TaskResponse::$response_name(content) = task.receive_response() {
									classify!{
										the name using its content $(with $response)?
										as either one of $successes or $failures
									};
								}
							} else {
								has_skipped_initial_output = true;
							}
						}
					}

					response = MaintainerResponse::$response_name($successes, $failures);
				}
			};
		}

		match response {
			MaintainerResponse::Output(mut successes, mut failures) => receive_other!{
				Output with response
				that will be classified as either one of the successes or failures
			},
			MaintainerResponse::Check(mut successes, mut failures) => receive_other!{
				Check that will be classified as either one of the successes or failures
			},
			MaintainerResponse::Stop(mut successes, mut failures) => receive_other!{
				Stop that will be classified as either one of the successes or failures
			},
			_ => todo!()
		}

		response
	}
}

#[cfg(test)]
mod t {
	use crate::process::fake_process::{FAKE_OUTPUT_CONTENT, request};
	use super::{MaintainerRequest as Request, MaintainerResponse as Response, TaskMaintainer};

	fn create_maintainer<'a>(fake_success: &'a str, fake_failure: &'a str)
	-> (TaskMaintainer<'a>, Vec<&'a [u8]>) {
		let task_names = vec![&b"success"[..], &b"failure"[..]];
		let mut maintainer = TaskMaintainer::new();
		maintainer.create(task_names[0], fake_success.as_bytes()).unwrap();
		maintainer.create(task_names[1], fake_failure.as_bytes()).unwrap();
		(maintainer, task_names)
	}

	#[test]
	fn can_receive_output_response() {
		let (maintainer, task_names) = create_maintainer(
			request::OUTPUT_SUCCESS,
			request::OUTPUT_FAILURE);
		let max_output_size = 30;

		maintainer.send_request(Request::Output(max_output_size, task_names.clone()));
		let response = maintainer.receive_response(task_names.clone());

		assert_eq!(response, Response::Output(
			vec![
				(task_names[0], FAKE_OUTPUT_CONTENT.into_iter().cycle().take(max_output_size).collect())
			],
			vec![task_names[1]]));
	}

	macro_rules! expected_response {
		($response_name:ident) => {
			Response::$response_name(vec![&b"success"[..]], vec![&b"failure"[..]])
		};
	}

	#[test]
	fn can_receive_check_response() {
		let (maintainer, task_names) = create_maintainer(
			request::CHECK_SUCCESS,
			request::CHECK_FAILURE);

		maintainer.send_request(Request::Check(task_names.clone()));
		let response = maintainer.receive_response(task_names.clone());

		assert_eq!(response, expected_response!(Check));
	}

	#[test]
	fn can_receive_stop_response() {
		let (maintainer, task_names) = create_maintainer(
			request::STOP_SUCCESS,
			request::STOP_FAILURE);

		maintainer.send_request(Request::Stop(task_names.clone()));
		let response = maintainer.receive_response(task_names.clone());

		assert_eq!(response, expected_response!(Stop));
	}
}
