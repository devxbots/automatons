pub type Steps = Vec<Box<dyn Fn() -> Box<dyn Step>>>;

pub struct Task {
    steps: Steps,
}

impl Task {
    pub fn new(steps: Steps) -> Self {
        Self { steps }
    }

    pub fn steps(&self) -> &Steps {
        &self.steps
    }
}

pub trait Step {
    fn init() -> Box<dyn Step>
    where
        Self: Sized;

    fn execute(&mut self) -> Transition;
    fn executed(&self) -> bool;
}

pub enum Transition {
    Next,
    Failure,
    Complete,
}
