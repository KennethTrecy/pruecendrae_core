#[derive(Debug, PartialEq)]
pub enum Request<'a> {
	Output(usize, Vec<&'a str>),
	Start(Vec<&'a str>),
	Check(Vec<&'a str>),
	Stop(Vec<&'a str>),
	Kill(Vec<&'a str>)
}
