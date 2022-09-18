use anyhow::Context;
use async_trait::async_trait;

use automatons::{Automaton, Error, State, Task, Transition};

#[tokio::test]
async fn test() -> Result<(), Error> {
    let automaton = HelloWorld;
    let state = automaton.execute().await?;

    assert_eq!("Hello, World!", state.get::<Message>().unwrap().0);

    Ok(())
}

// Return type
#[derive(Debug)]
struct Message(String);

// Automaton
#[derive(Debug)]
struct HelloWorld;

// Task
struct Hello;

// Task
struct World;

impl Automaton for HelloWorld {
    fn initial_task(&self) -> Box<dyn Task> {
        Box::new(Hello)
    }
}

#[async_trait]
impl Task for Hello {
    async fn execute(&mut self, state: &mut State) -> Result<Transition, Error> {
        state.insert(Message(String::from("Hello")));
        Ok(Transition::Next(Box::new(World)))
    }
}

#[async_trait]
impl Task for World {
    async fn execute(&mut self, state: &mut State) -> Result<Transition, Error> {
        let message: &mut Message = state
            .get_mut()
            .context("failed to get message from state")?;

        message.0.push_str(", World!");

        Ok(Transition::Complete)
    }
}
