use thiserror::Error;

use crate::task::{Task, Transition};

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
}

pub struct Engine;

impl Engine {
    pub fn run(task: &Task) -> Result<(), Error> {
        for step_init_fn in task.steps().iter() {
            let mut step = step_init_fn();
            let transition = step.execute();

            match transition {
                Transition::Next => continue,
                Transition::Failure => return Err(Error::Unknown),
                Transition::Complete => break,
            }
        }

        Ok(())
    }
}
