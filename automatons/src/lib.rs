use std::fmt::Debug;

use async_trait::async_trait;

pub use crate::error::Error;
pub use crate::state::State;
pub use crate::task::{Task, Transition};

mod error;
mod state;
mod task;

/// Trait for automatons
///
/// Automatons execute a series of tasks. This trait defines the behavior that automatons must
/// implement so that they can be executed inside a runtime.
#[async_trait]
pub trait Automaton: Debug {
    /// Returns the initial state for the execution.
    ///
    /// Automatons can customize the state that is passed to the tasks. This can be useful to inject
    /// secrets or configuration on which tasks might depend. By default, an empty state is used.
    fn initial_state(&self) -> State {
        State::new()
    }

    /// Returns the first task in the automaton.
    ///
    /// The initial task defines the entry point of the automaton. This task will be executed with
    /// the initial state.
    fn initial_task(&self) -> Box<dyn Task>;

    /// Returns the task that is called after the completed transition.
    ///
    /// When a tasks returns `Transition::Complete` to end the execution of the automaton, a final
    /// task is executed. This can be useful to perform any teardown actions, for example to remove
    /// resources that were created in a previous step. By default, a "noop" task is executed that
    /// performs no action.
    fn complete_task(&self) -> Box<dyn Task> {
        Box::new(NoopTask)
    }

    /// Executes the automaton.
    ///
    /// Automatons execute a series of tasks. When started, the automaton first initializes a new
    /// state. Then, it iterates over the list of tasks. It initializes and executes each task one
    /// by one until it either reaches the end of the list or a task returns `Transition::Complete`.
    /// In both instances, the task returned by the `complete_task` method is executed and the
    /// automaton shuts down.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn execute(&self) -> Result<State, Error> {
        let mut state = self.initial_state();
        let mut task = self.initial_task();

        loop {
            task = match task.execute(&mut state).await? {
                Transition::Next(task) => task,
                Transition::Complete => break,
            }
        }

        let mut complete_task = self.complete_task();
        complete_task.execute(&mut state).await?;

        Ok(state)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct NoopTask;

#[async_trait]
impl Task for NoopTask {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn execute(&mut self, _state: &mut State) -> Result<Transition, Error> {
        Ok(Transition::Complete)
    }
}

#[cfg(test)]
mod tests {
    use super::NoopTask;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<NoopTask>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<NoopTask>();
    }
}
