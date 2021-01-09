#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Response<'a> {
	Output(Vec<(&'a str, Vec<u8>)>, Vec<&'a str>),
	Start(Vec<&'a str>, Vec<&'a str>),
	Check(Vec<&'a str>, Vec<&'a str>),
	Stop(Vec<&'a str>, Vec<&'a str>),
	Kill(Vec<&'a str>, Vec<&'a str>)
}

impl<'a> From<Response<'a>> for String {
	fn from(response: Response) -> Self {
		macro_rules! append_group {
			($units:ident named as $group:literal to $response:ident) => {
				if $units.len() > 0 {
					$response += &format!("\t{}\n", $group);
					for unit in $units {
						$response += &format!("\t\t{}|\n", unit);
					}
				}
			};
		}

		macro_rules! append_groups {
			($successes:ident and $failures:ident known as $response_name:literal) => {
				let mut response = format!("{}\n", $response_name);

				append_group!($successes named as "successes" to response);
				append_group!($failures named as "failures" to response);

				response
			};
		}

		match response {
			Response::Output(successes, failures) => {
				let mut response = String::from("output\n");

				if successes.len() > 0 {
					response += "\tsuccesses\n";
					for success in successes {
						let (name, content) = success;
						response += &format!("\t\t{}\n", name);
						response += "\t\t\t===\n";
						response += &format!("{}\n", String::from_utf8(content).unwrap());
						response += "\t\t\t===\n";
					}
				}

				append_group!(failures named as "failures" to response);

				response
			},
			Response::Check(successes, failures) => {
				append_groups!{successes and failures known as "check"}
			},
			Response::Stop(successes, failures) => {
				append_groups!{successes and failures known as "stop"}
			},
			_ => todo!()
		}
	}
}

#[cfg(test)]
mod t {
	use super::Response;

	macro_rules! test {
		(
			$test_name:ident
			from $response:ident($successes:expr, $failures:expr)
			to $expected_string:expr
		) => {
			#[test]
			fn $test_name() {
				let response = Response::$response($successes, $failures);

				let response: String = response.into();

				assert_eq!(response, $expected_string);
			}
		};
	}

	test!{
		can_convert_output
		from Output(vec![("task A", b"task A contents".to_vec())], vec!["task B"])
		to "output\n\tsuccesses\n\t\ttask A
			===\ntask A contents
			===\n\tfailures\n\t\ttask B|\n"
	}

	test!{
		can_convert_stop
		from Stop(vec!["task C", "task D"], vec!["task E"])
		to "stop\n\tsuccesses\n\t\ttask C|\n\t\ttask D|\n\tfailures\n\t\ttask E|\n"
	}

	test!{
		can_convert_check
		from Check(vec!["task F"], vec!["task G", "task H"])
		to "check\n\tsuccesses\n\t\ttask F|\n\tfailures\n\t\ttask G|\n\t\ttask H|\n"
	}
}
