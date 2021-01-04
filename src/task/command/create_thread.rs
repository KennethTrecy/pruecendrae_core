use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use crate::request::Request;
use crate::response::Response;
use crate::task::command::run;
use crate::task::process::Process;

pub fn create_thread(program: String, arguments: Vec<String>)
-> (JoinHandle<()>, Sender<Request>, Receiver<Response>) {
	let (exsender, inreceiver) = mpsc::channel();
	let (insender, exreceiver) = mpsc::channel();

	let thread = thread::spawn(move || {
		let mut process = run(program.clone(), arguments.clone());

		for request in inreceiver.iter() {
			let mut may_continue = true;
			let response;
			match request {
				Request::Output(max_output_size) => {
					let mut output = vec![0; max_output_size];
					let read_size = process.read(&mut output).unwrap();
					let output = (&output[0..read_size]).to_vec();
					response = Response::Output(output);
				},
				Request::Check => {
					response = if process.check() {
						Response::Running
					} else {
						Response::SuccessStop
					}
				},
				Request::Start => {
					response = if process.check() {
						Response::FailedStart
					} else {
						process = run(program.clone(), arguments.clone());
						Response::SuccessStart
					}
				},
				Request::Stop => {
					match process.stop() {
						Ok(()) => response = Response::SuccessStop,
						Err(_) => response = Response::FailedStop
					}
				},
				Request::Kill => {
					process.stop().unwrap();
					may_continue = false;
					response = Response::Killed;
				}
			}

			insender.send(response).unwrap();

			if !may_continue { break; }
		}
	});

	(thread, exsender, exreceiver)
}
