use crate::Response as MaintainerResponse;
use crate::task::Response as TaskResponse;
use super::TaskMaintainer;

impl<'a> TaskMaintainer<'a> {
	pub(super) fn receive_initial_response(&self, names: &Vec<&'a [u8]>) -> MaintainerResponse<'a> {
		let mut name_iterator = names.iter();

		loop {
			let name = name_iterator.next();
			if let Some(&name) = name {
				if let Some(task) = self.tasks.get(name) {
					let response;

					macro_rules! receive_initial_reponse {
						($response_name:ident $content:ident $(with $response:ident)?) => {
							{
								let mut successful_responses = Vec::new();
								let mut failed_responses = Vec::new();

								classify!{
									the name using its $content $(with $response)?
									as either one of successful_responses or failed_responses
								}

								response = MaintainerResponse::$response_name(
									successful_responses, failed_responses);
							}
						};
					}

					match task.receive_response() {
						TaskResponse::Output(content) => {
							receive_initial_reponse!(Output content with response);
						},
						TaskResponse::Check(content) => receive_initial_reponse!{Check content},
						TaskResponse::Start(_content) => { todo!() },
						TaskResponse::Stop(content) => receive_initial_reponse!{Stop content},
						TaskResponse::Kill(_content) => { todo!() }
					}

					break response;
				} else {
					todo!()
				}
			}
		}
	}
}

#[cfg(test)]
mod t {
	use crate::Request;
	use crate::process::fake_process::{FAKE_OUTPUT_CONTENT, request};
	use super::{MaintainerResponse as Response, TaskMaintainer};

	#[test]
	fn can_receive_initial_response() {
		let task_names = vec![&b"request a"[..], &b"request b"[..], &b"request c"[..]];
		let max_output_size = 20;
		let mut maintainer = TaskMaintainer::new();
		maintainer.create(task_names[0], request::OUTPUT_SUCCESS.as_bytes()).unwrap();
		maintainer.create(task_names[1], request::OUTPUT_SUCCESS.as_bytes()).unwrap();
		maintainer.create(task_names[2], request::OUTPUT_SUCCESS.as_bytes()).unwrap();

		maintainer.send_request(Request::Output(max_output_size, task_names.clone()));
		let initial_response = maintainer.receive_initial_response(&task_names);

		assert_eq!(initial_response, Response::Output(vec![
			(task_names[0], FAKE_OUTPUT_CONTENT.take(max_output_size).collect())
		], Vec::new()))
	}
}
