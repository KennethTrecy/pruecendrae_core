#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum SuccessResponse<'a> {
	Output(Vec<(&'a [u8], Vec<u8>)>),
	Start(Vec<&'a [u8]>),
	Running(Vec<&'a [u8]>),
	Stop(Vec<&'a [u8]>),
	Killed(Vec<&'a [u8]>)
}
