macro_rules! classify {
	(
		the $name:ident using its $content:ident
		as either one of $successes:ident or $failures:ident
	) => {
		match $content {
			Ok(()) => $successes.push($name),
			Err(()) => $failures.push($name)
		}
	};
	(
		the $name:ident using its $content:ident with $response:ident
		as either one of $successes:ident or $failures:ident
	) => {
		match $content {
			Ok($response) => $successes.push(($name, $response)),
			Err(()) => $failures.push($name)
		}
	};
}
