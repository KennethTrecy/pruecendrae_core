use crate::{Request as MaintainerRequest, Response as MaintainerResponse};
use crate::task::{Request as TaskRequest, Response as TaskResponse};
use super::TaskMaintainer;

impl<'a> TaskMaintainer<'a> {
	pub fn send_request(&self, request_type: MaintainerRequest<'a>) {
		macro_rules! request {
			(for each $names:ident, $name:ident with $($arguments:ident)*) => {
				{
					for name in $names {
						if self.tasks.contains_key(name) {
							let task = self.tasks.get(name).unwrap();
							task.send_request(TaskRequest::$name($($arguments,)*)).unwrap();
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
			_ => { todo!() }
		}
	}

	pub fn receive_response(&self, names: Vec<&'a [u8]>) -> MaintainerResponse<'a> {
		let mut response = self.receive_initial_response(&names);
		match response {
			MaintainerResponse::Output(mut successes, mut failures) => {
				let mut has_skipped_initial_output = false;
				for name in names {
					if let Some(task) = self.tasks.get(name) {
						if has_skipped_initial_output {
							if let TaskResponse::Output(content) = task.receive_response() {
								match content {
									Ok(response) => successes.push((name, response)),
									Err(()) => failures.push(name)
								}
							}
						} else {
							has_skipped_initial_output = true;
						}
					}
				}

				response = MaintainerResponse::Output(successes, failures);
			},
			_ => todo!()
		}

		response
	}
}

#[cfg(test)]
mod t {
	use crate::process::fake_process::FAKE_SAMPLE;
	use super::{MaintainerRequest as Request, MaintainerResponse as Response, TaskMaintainer};

	fn create_maintainer<'a>(fake_success: &'a [u8], fake_failure: &'a [u8])
	-> (TaskMaintainer<'a>, Vec<&'a [u8]>) {
		let task_names = vec![&b"success"[..], &b"failure"[..]];
		let mut maintainer = TaskMaintainer::new();
		maintainer.create(task_names[0], fake_success).unwrap();
		maintainer.create(task_names[1], fake_failure).unwrap();
		(maintainer, task_names)
	}

	#[ignore]
	#[test]
	fn can_receive_output_response() {
		let (maintainer, task_names) = create_maintainer(
			b"request output_success",
			b"request output_failure");
		let max_output_size = 30;

		maintainer.send_request(Request::Output(max_output_size, task_names.clone()));
		let response = maintainer.receive_response(task_names.clone());

		assert_eq!(response, Response::Output(
			vec![(task_names[0], FAKE_SAMPLE.take(max_output_size).collect())],
			vec![task_names[1]]
		));
	}
}
