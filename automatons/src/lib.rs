use std::fmt::Debug;

use async_trait::async_trait;

pub use crate::error::Error;
pub use crate::task::{Task, Transition};

mod error;
mod task;

/// Trait for the output of an automaton
///
/// Automatons can produce something and return it to their caller. These products must implement
/// this marker trait.
pub trait Product: Send + Sync {}

/// Trait for automatons
///
/// Automatons execute a series of tasks. This trait defines the behavior that automatons must
/// implement so that they can be executed inside a runtime.
#[async_trait]
pub trait Automaton<P: Product>: Debug {
    /// Returns the first task in the automaton.
    ///
    /// The initial task defines the entry point of the automaton. This task will be executed with
    /// the initial state.
    fn initial_task(&self) -> Box<dyn Task<P>>;

    /// Returns the task that is called after the completed transition.
    ///
    /// When a tasks returns `Transition::Complete` to end the execution of the automaton, a final
    /// task is executed. This can be useful to perform any teardown actions, for example to remove
    /// resources that were created in a previous step. By default, a "noop" task is executed that
    /// performs no action.
    fn complete_task(&self) -> Option<Box<dyn Task<P>>> {
        None
    }

    /// Executes the automaton.
    ///
    /// Automatons execute a series of tasks. When started, the automaton first initializes a new
    /// state. Then, it iterates over the list of tasks. It initializes and executes each task one
    /// by one until it either reaches the end of the list or a task returns `Transition::Complete`.
    /// In both instances, the task returned by the `complete_task` method is executed and the
    /// automaton shuts down.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn execute(&self) -> Result<P, Error> {
        let mut automaton_output;
        let mut task = self.initial_task();

        loop {
            task = match task.execute().await? {
                Transition::Next(task) => task,
                Transition::Complete(output) => {
                    automaton_output = output;
                    break;
                }
            }
        }

        if let Some(mut complete_task) = self.complete_task() {
            if let Transition::Complete(output) = complete_task.execute().await? {
                automaton_output = output;
            }
        }

        Ok(automaton_output)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct NoopTask;

#[async_trait]
impl Task<()> for NoopTask {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn execute(&mut self) -> Result<Transition<()>, Error> {
        Ok(Transition::Complete(()))
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
