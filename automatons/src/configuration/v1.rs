use serde::{Deserialize, Serialize};

use crate::steps::Step;

#[derive(Deserialize, Serialize)]
pub struct V1 {
    steps: Vec<Box<dyn Step>>,
}

impl V1 {
    pub fn steps(&self) -> &Vec<Box<dyn Step>> {
        &self.steps
    }
}
