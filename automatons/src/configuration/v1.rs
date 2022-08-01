use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct V1 {
    steps: Vec<String>,
}

impl V1 {
    pub fn steps(&self) -> &Vec<String> {
        &self.steps
    }
}
