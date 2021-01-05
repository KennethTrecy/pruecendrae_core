use crate::{Request as MaintainerRequest, Response as MaintainerResponse};
use crate::task::{Request as TaskRequest, Response as TaskResponse};
use super::TaskMaintainer;

impl<'a> TaskMaintainer<'a> {
	pub(super) fn receive_initial_response(&self, names: &Vec<&'a [u8]>) -> MaintainerResponse<'a> {
		let mut name_iterator = names.iter();
		loop {
			let name = name_iterator.next();
			if let Some(_) = name {
				let name = name.unwrap();
				if self.tasks.contains_key(name) {
					let task = self.tasks.get(name).unwrap();
					let response;
					match task.receive_response() {
						TaskResponse::Output(content) => {
							let mut successful_responses = Vec::new();
							let mut failed_responses = Vec::new();

							match content {
								Ok(response) => successful_responses.push((*name, response)),
								Err(()) => failed_responses.push(*name)
							}

							response = MaintainerResponse::Output(successful_responses, failed_responses);
						},
						_ => todo!()
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
	use crate::process::fake_process::FAKE_SAMPLE;

	use super::{MaintainerRequest as Request, MaintainerResponse as Response, TaskMaintainer};

	#[test]
	fn can_receive_initial_response() {
		let task_names = vec![&b"request a"[..], &b"request b"[..], &b"request c"[..]];
		let max_output_size = 20;
		let mut maintainer = TaskMaintainer::new();
		maintainer.create(task_names[0], b"request output_success").unwrap();
		maintainer.create(task_names[1], b"request output_success").unwrap();
		maintainer.create(task_names[2], b"request output_success").unwrap();

		maintainer.send_request(Request::Output(max_output_size, task_names.clone()));
		let initial_response = maintainer.receive_initial_response(&task_names);

		assert_eq!(initial_response, Response::Output(vec![
			(task_names[0], FAKE_SAMPLE.take(max_output_size).collect())
		], Vec::new()))
	}
}
