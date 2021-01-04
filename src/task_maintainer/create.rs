use crate::Task;
use super::TaskMaintainer;

impl<'a> TaskMaintainer<'a> {
	/// Creates a task and adds to the collection.
	pub fn create(&mut self, name: &'a [u8], command: &'a [u8]) -> Result<(), ()> {
		let task = Task::new(name, command);
		if self.tasks.contains_key(name) {
			Err(())
		} else {
			self.tasks.insert(name, task);
			Ok(())
		}
	}
}

#[cfg(test)]
mod t {
	use super::TaskMaintainer;

	#[test]
	fn can_create_new_task() {
		let mut maintainer = TaskMaintainer::new();

		let result = maintainer.create(b"new task", b"task a");

		assert_eq!(result, Ok(()));
	}

	#[test]
	fn cannot_recreate_same_task() {
		let mut maintainer = TaskMaintainer::new();
		maintainer.create(b"same task name", b"task b").unwrap();

		let result = maintainer.create(b"same task name", b"task b");

		assert_eq!(result, Err(()));
	}


}
