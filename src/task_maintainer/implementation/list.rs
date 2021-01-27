use super::TaskMaintainer;

impl TaskMaintainer {
	/// Lists the stored tasks
	pub fn list(&self) -> Vec<(&str, &str)> {
		self.tasks.iter().map(|(name, task)| (name.as_str(), task.command())).collect()
	}
}

#[cfg(test)]
mod t {
	use super::TaskMaintainer;

	#[test]
	fn can_list_tasks() {
		let mut maintainer = TaskMaintainer::new();
		maintainer.create("new task c", b"task c").unwrap();
		maintainer.create("new task d", b"task d").unwrap();

		let keys = maintainer.list();

		assert_eq!(keys, vec![
			("new task c", "task c"),
			("new task d", "task d")
		]);
	}
}
