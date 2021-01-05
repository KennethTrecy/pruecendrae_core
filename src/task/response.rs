use crate::task::result::Result;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Response {
	Output(Result<Vec<u8>>),
	Start(Result<()>),
	Check(Result<()>),
	Stop(Result<()>),
	Killed(Result<()>)
}
