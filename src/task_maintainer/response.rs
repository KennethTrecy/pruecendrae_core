#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Response<'a> {
	Output(Vec<(&'a [u8], Vec<u8>)>),
	SuccessStart(Vec<&'a [u8]>),
	FailedStart(Vec<&'a [u8]>),
	Running(Vec<&'a [u8]>),
	SuccessStop(Vec<&'a [u8]>),
	FailedStop(Vec<&'a [u8]>),
	Killed(Vec<&'a [u8]>)
}
