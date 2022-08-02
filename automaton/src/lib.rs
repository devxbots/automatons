use async_trait::async_trait;

pub use crate::error::Error;
pub use crate::state::State;
pub use crate::task::{Task, Transition};

mod error;
mod state;
mod task;

/// List of tasks
///
/// Automatons execute a series of tasks. The tasks are provided as a list of constructor functions
/// so that the automaton can initialize the tasks one by one.
pub type Tasks = Vec<Box<dyn Fn(&State) -> Box<dyn Task> + Send + Sync>>;

/// Trait for automatons
///
/// Automatons execute a series of tasks. This trait defines the behavior that automatons must
/// implement so that they can be executed inside a runtime.
#[async_trait]
pub trait Automaton {
    /// Returns the tasks of the automaton.
    ///
    /// Automatons execute a series of tasks. They provide the engine a list of constructor
    /// functions. This enables the engine to initialize each step at the right time.
    fn tasks(&self) -> Tasks;

    /// Returns the initial state for the execution.
    ///
    /// Automatons can customize the state that is passed to the tasks. This can be useful to inject
    /// secrets or configuration on which tasks might depend. By default, an empty state is used.
    fn initial_state(&self) -> State {
        State::new()
    }

    /// Returns the task that is called after the completed transition.
    ///
    /// When a tasks returns `Transition::Complete` to end the execution of the automaton, a final
    /// task is executed. This can be useful to perform any teardown actions, for example to remove
    /// resources that were created in a previous step. By default, a "noop" task is executed that
    /// performs no action.
    fn complete_task(&self, state: &State) -> Box<dyn Task> {
        NoopTask::init(state)
    }

    /// Executes the automaton.
    ///
    /// Automatons execute a series of tasks. When started, the automaton first initializes a new
    /// state. Then, it iterates over the list of tasks. It initializes and executes each task one
    /// by one until it either reaches the end of the list or a task returns `Transition::Complete`.
    /// In both instances, the task returned by the `complete_task` method is executed and the
    /// automaton shuts down.
    async fn execute(&self) -> Result<State, Error> {
        let mut state = self.initial_state();

        for task_init_fn in self.tasks().iter() {
            let mut task = task_init_fn(&state);
            let transition = task.execute(&mut state).await?;

            match transition {
                Transition::Next => continue,
                Transition::Complete => break,
            }
        }

        let mut complete_task = self.complete_task(&state);
        complete_task.execute(&mut state).await?;

        Ok(state)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct NoopTask;

#[async_trait]
impl Task for NoopTask {
    fn init(_state: &State) -> Box<dyn Task>
    where
        Self: Sized,
    {
        Box::new(NoopTask)
    }

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
