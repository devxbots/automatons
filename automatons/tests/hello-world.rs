use async_trait::async_trait;

use automatons::{Automaton, Error, Product, Task, Transition};

#[tokio::test]
async fn test() -> Result<(), Error> {
    let automaton = HelloWorld;
    let message = automaton.execute().await?;

    assert_eq!("Hello, World!", message.0);

    Ok(())
}

// Product
struct Message(String);
impl Product for Message {}

// Automaton
#[derive(Debug)]
struct HelloWorld;

// Task
struct Hello {}

// Task
struct World {
    props: String,
}

impl Automaton<Message> for HelloWorld {
    fn initial_task(&self) -> Box<dyn Task<Message>> {
        Box::new(Hello {})
    }
}

#[async_trait]
impl Task<Message> for Hello {
    async fn execute(&mut self) -> Result<Transition<Message>, Error> {
        Ok(Transition::Next(Box::new(World {
            props: String::from("Hello"),
        })))
    }
}

#[async_trait]
impl Task<Message> for World {
    async fn execute(&mut self) -> Result<Transition<Message>, Error> {
        Ok(Transition::Complete(Message(format!(
            "{}, World!",
            self.props
        ))))
    }
}
