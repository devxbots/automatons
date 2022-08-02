use automatons::engine::Engine;
use automatons::task::{Step, Task, Transition};

struct Hello {}
struct World {}

impl Step for Hello {
    fn init() -> Box<dyn Step>
    where
        Self: Sized,
    {
        Box::new(Hello {})
    }

    fn execute(&mut self) -> Transition {
        Transition::Next
    }
}

impl Step for World {
    fn init() -> Box<dyn Step>
    where
        Self: Sized,
    {
        Box::new(World {})
    }

    fn execute(&mut self) -> Transition {
        Transition::Complete
    }
}

#[test]
fn hello_world() {
    let task = Task::new(vec![Box::new(Hello::init), Box::new(World::init)]);

    let result = Engine::run(&task);

    assert!(result.is_ok());
}
