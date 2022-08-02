use automatons::task::{Step, Task, Transition};

struct Hello {
    executed: bool,
}

struct World {
    executed: bool,
}

impl Step for Hello {
    fn init() -> Box<dyn Step>
    where
        Self: Sized,
    {
        Box::new(Hello { executed: false })
    }

    fn execute(&mut self) -> Transition {
        self.executed = true;

        Transition::Next
    }

    fn executed(&self) -> bool {
        self.executed
    }
}

impl Step for World {
    fn init() -> Box<dyn Step>
    where
        Self: Sized,
    {
        Box::new(World { executed: false })
    }

    fn execute(&mut self) -> Transition {
        self.executed = true;

        Transition::Complete
    }

    fn executed(&self) -> bool {
        self.executed
    }
}

#[test]
fn hello_world() {
    let task = Task::new(vec![Box::new(Hello::init), Box::new(World::init)]);

    let hello_init_fn = task.steps().get(0).unwrap();
    let mut hello = hello_init_fn();

    let transition = hello.execute();
    assert!(matches!(transition, Transition::Next));
    assert!(hello.executed());

    let world_init_fn = task.steps().get(1).unwrap();
    let mut world = world_init_fn();

    let transition = world.execute();
    assert!(matches!(transition, Transition::Complete));
    assert!(world.executed());
}
