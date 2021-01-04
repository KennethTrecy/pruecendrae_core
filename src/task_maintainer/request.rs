pub enum Request<'a> {
	Output(usize, Vec<&'a [u8]>),
	Start(Vec<&'a [u8]>),
	Check(Vec<&'a [u8]>),
	Stop(Vec<&'a [u8]>),
	Kill(Vec<&'a [u8]>)
}
