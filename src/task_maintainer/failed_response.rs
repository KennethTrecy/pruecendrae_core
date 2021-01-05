#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum FailedResponse<'a> {
	Start(Vec<&'a [u8]>),
	Stop(Vec<&'a [u8]>),
	AlreadyStopped(Vec<&'a [u8]>)
}
