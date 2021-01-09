#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Response<'a> {
	Output(Vec<(&'a str, Vec<u8>)>, Vec<&'a str>),
	Start(Vec<&'a str>, Vec<&'a str>),
	Check(Vec<&'a str>, Vec<&'a str>),
	Stop(Vec<&'a str>, Vec<&'a str>),
	Kill(Vec<&'a str>, Vec<&'a str>)
}
