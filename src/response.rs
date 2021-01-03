#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Response {
	Output(Vec<u8>),
	SuccessTermination,
	ErrorTermination
}
