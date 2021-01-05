use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use crate::process::Process;
use crate::task::command::run;
use crate::task::request::Request;
use crate::task::response::Response;

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
					response = Response::Output(Ok(output));
				},
				Request::Check => {
					let result = if process.check() {
						Ok(())
					} else {
						Err(())
					};

					response = Response::Check(result);
				},
				Request::Start => {
					let result = if process.check() {
						Ok(())
					} else {
						process = run(program.clone(), arguments.clone());
						Err(())
					};

					response = Response::Start(result);
				},
				Request::Stop => {
					let result = process.stop().map_err(|_| ());
					response = Response::Stop(result);
				},
				Request::Kill => {
					let result = process.stop().map_err(|_| ());
					may_continue = false;
					response = Response::Killed(result);
				}
			}

			insender.send(response).unwrap();

			if !may_continue { break; }
		}
	});

	(thread, exsender, exreceiver)
}
