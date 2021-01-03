pub fn parse_command(command: &[u8]) -> (String, Vec<String>) {
	let mut program = None;
	let mut arguments = Vec::new();
	let mut start = 0;
	let mut end = 0;
	let command_length = command.len();

	while end <= command_length {
		if end == command_length || command[end] == ' ' as u8 {
			let fragment = &command[start..end];
			let fragment = String::from_utf8_lossy(&fragment[..]).into_owned();

			match program {
				Some(_) => arguments.push(fragment),
				None => program = Some(fragment)
			}

			end += 1;
			start = end;
		}

		end += 1;
	}

	(program.unwrap(), arguments)
}

#[cfg(test)]
mod t {
	use super::parse_command;

	#[test]
	fn can_parse_command() {
		let sample = b"program argument_1 argument_2 argument_3";
		let expected_program = String::from("program");
		let expected_arguments = vec![
			String::from("argument_1"),
			String::from("argument_2"),
			String::from("argument_3")
		];

		let command = parse_command(&sample[..]);

		assert_eq!(command, (expected_program, expected_arguments));
	}
}
