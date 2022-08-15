use std::future::Future;
use std::process::Output;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, sync_channel, SyncSender};
use tokio::runtime::Runtime;
use crate::{System};

pub struct Task<O: Send + 'static> {
	runtime: Arc<Runtime>,
	receiver: Receiver<O>,
	sender: SyncSender<O>,
	in_progress: bool,
}

impl<O: Send + 'static> Task<O> {
	pub fn new(runtime: &Arc<Runtime>) -> Task<O> {
		let (sender, receiver) = sync_channel(1);
		Task {
			runtime: runtime.clone(),
			receiver,
			sender,
			in_progress: false,
		}
	}

	pub fn launch<F>(&mut self, func: F) -> Result<(), TaskAlreadyInProgress>
		where
			F: 'static + Send + Future<Output=O>,

	{
		if self.in_progress {
			return Err(TaskAlreadyInProgress {});
		}

		self.in_progress = true;
		let sender = self.sender.clone();
		self.runtime.spawn(async move {
			let output: O = func.await;
			sender.send(output).unwrap();
		});

		Ok(())
	}

	pub fn in_progress(&self) -> bool {
		self.in_progress
	}

	pub fn poll(&mut self) -> Option<O> {
		if !self.in_progress {
			return None;
		}
		let option = self.receiver.try_recv().ok();
		if option.is_some() {
			self.in_progress = false;
		}
		option
	}
}

#[derive(Debug)]
pub struct TaskAlreadyInProgress {

}