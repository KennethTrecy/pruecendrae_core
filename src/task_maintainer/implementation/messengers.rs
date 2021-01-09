use crate::{Request as MaintainerRequest, Response as MaintainerResponse};
use crate::task::{Request as TaskRequest, Response as TaskResponse};
use super::TaskMaintainer;

impl TaskMaintainer {
	pub fn send_request<'a>(&self, request_type: MaintainerRequest<'a>) -> Vec<&'a str> {
		let mut requested_names;

		macro_rules! request {
			(for each $names:ident, $name:ident $(with $($arguments:ident)+)?) => {
				{
					requested_names = Vec::with_capacity($names.len());
					for name in $names {
						if self.tasks.contains_key(name) {
							requested_names.push(name);
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
			MaintainerRequest::Start(_names) => { todo!() },
			MaintainerRequest::Stop(names) => request!{for each names, Stop},
			MaintainerRequest::Kill(_names) => { todo!() }
		}

		requested_names
	}

	pub fn receive_response<'a>(&self, names: Vec<&'a str>) -> MaintainerResponse<'a> {
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
			MaintainerResponse::Start(mut _successes, mut _failures) => {
				todo!()
			},
			MaintainerResponse::Stop(mut successes, mut failures) => receive_other!{
				Stop that will be classified as either one of the successes or failures
			},
			MaintainerResponse::Kill(mut _successes, mut _failures) => {
				todo!()
			}
		}

		response
	}
}

#[cfg(test)]
mod t {
	use crate::process::fake_process::{FAKE_OUTPUT_CONTENT, request};
	use super::{MaintainerRequest as Request, MaintainerResponse as Response, TaskMaintainer};

	fn create_maintainer<'a>(fake_success: &'a str, fake_failure: &'a str)
	-> (TaskMaintainer, Vec<&'a str>) {
		let task_names = vec!["success", "failure"];
		let mut maintainer = TaskMaintainer::new();
		maintainer.create(task_names[0], fake_success.as_bytes()).unwrap();
		maintainer.create(task_names[1], fake_failure.as_bytes()).unwrap();
		(maintainer, task_names)
	}

	macro_rules! test {
		(
			$test_name:ident
			with $($value:literal as $name:ident,)? $success:ident and $failure:ident
			used in $response_name:ident
			expecting $expected_response:expr
		) => {
			#[test]
			fn $test_name() {
				let (maintainer, task_names) = create_maintainer(
					request::$success,
					request::$failure);
				$(let $name = $value;)*

				maintainer.send_request(Request::$response_name($($name,)* task_names.clone()));
				let response = maintainer.receive_response(task_names.clone());

				assert_eq!(response, $expected_response);
			}
		};
	}

	test!{
		can_receive_output_response
		with 30 as max_output_size, OUTPUT_SUCCESS and OUTPUT_FAILURE
		used in Output
		expecting Response::Output(
			vec![
				("success", FAKE_OUTPUT_CONTENT.into_iter().cycle().take(max_output_size).collect())
			],
			vec!["failure"]
		)
	}

	macro_rules! expected_response {
		($response_name:ident) => {
			Response::$response_name(vec!["success"], vec!["failure"])
		};
	}

	test!{
		can_receive_check_response
		with CHECK_SUCCESS and CHECK_FAILURE
		used in Check
		expecting expected_response!(Check)
	}

	test!{
		can_receive_stop_response
		with STOP_SUCCESS and STOP_FAILURE
		used in Stop
		expecting expected_response!(Stop)
	}
}
