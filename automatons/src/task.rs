use async_trait::async_trait;

use crate::{Error, State};

/// Transition from one task to the next
///
/// When a task executes, it can control the transition to the next state in three different ways.
/// First, it can fail hard with an error. In this case, the runtime will stop execution and handle
/// the error gracefully. Second, a task can succeed and simply trigger the transition to the next
/// task. Third, a task can indicate that the automaton should finish early. This can be useful if
/// no work needs to be done.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Transition {
    /// Transition to the next task.
    Next,

    /// Skip all other tasks and go straight to the teardown task.
    Complete,
}

/// Executable task
///
/// Automatons execute a series of tasks. Each task has an initialization and an execution function.
/// Tasks can share data with each other by putting it into the shared state. If a task determines
/// that no more work needs to be done, it can complete the automaton early by returning a
/// [`Transition`] with the `Complete` variant.
#[async_trait]
pub trait Task: Send + Sync {
    /// Initializes a new instance of the task.
    ///
    /// Tasks are initialized by the engine when the previous task has finished successfully. If
    /// they need any data, they can retrieve it from the shared state.
    fn init(state: &State) -> Result<Box<dyn Task>, Error>
    where
        Self: Sized;

    /// Executes the task.
    ///
    /// Tasks can perform arbitrary units of work. They are executed asynchronously to avoid
    /// blocking the thread when waiting for external resources. Tasks return a [`Result`] with a
    /// [`Transition`], which tells the engine whether to continue, handle an unexpected failure, or
    /// return early since there is no more work to be done.
    async fn execute(&mut self, state: &mut State) -> Result<Transition, Error>;
}
