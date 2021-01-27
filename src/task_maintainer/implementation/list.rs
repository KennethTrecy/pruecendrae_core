use super::TaskMaintainer;

impl TaskMaintainer {
	/// Lists the stored tasks
	pub fn list(&self) -> Vec<&str> {
		self.tasks.keys().map(|string| string.as_str()).collect()
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

		assert_eq!(keys, vec!["new task c", "new task d"]);
	}
}
