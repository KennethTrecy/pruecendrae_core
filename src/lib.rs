//! # Feo Template
//! Please read the README.md for more information.

/// Contains the `Task` struct to run commands.
mod task;

/// Contains the basic types used in this whole module.
mod process;

/// Contains the `TaskMaintainer` struct to maintain tasks.
mod task_maintainer;

pub use task::Task;
pub use task_maintainer::{TaskMaintainer, Request, Response};
