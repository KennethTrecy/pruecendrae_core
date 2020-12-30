pub enum TaskMessage {
	RequestOutput,
	ResponseOutput(Vec<u8>)
}
