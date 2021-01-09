use crate::Task;
use super::TaskMaintainer;

impl TaskMaintainer {
	/// Creates a task and adds to the collection.
	pub fn create(&mut self, name: &str, command: &[u8]) -> Result<(), ()> {
		let task = Task::new(command);
		if self.tasks.contains_key(name) {
			Err(())
		} else {
			self.tasks.insert(String::from(name), task);
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

		let result = maintainer.create("new task", b"task a");

		assert_eq!(result, Ok(()));
	}

	#[test]
	fn cannot_recreate_same_task() {
		let mut maintainer = TaskMaintainer::new();
		maintainer.create("same task name", b"task b").unwrap();

		let result = maintainer.create("same task name", b"task b");

		assert_eq!(result, Err(()));
	}


}
