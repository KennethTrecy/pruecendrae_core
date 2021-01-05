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
							response = MaintainerResponse::Output(Vec::new(), Vec::new());
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
