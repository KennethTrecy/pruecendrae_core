#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Response<'a> {
	Output(Vec<(&'a [u8], Vec<u8>)>, Vec<&'a [u8]>),
	Start(Vec<&'a [u8]>, Vec<&'a [u8]>),
	Check(Vec<&'a [u8]>, Vec<&'a [u8]>),
	Stop(Vec<&'a [u8]>, Vec<&'a [u8]>),
	Kill(Vec<&'a [u8]>, Vec<&'a [u8]>)
}
