use serde::{Deserialize, Serialize};

use crate::steps::Step;

#[derive(Deserialize, Serialize)]
pub struct Example {
    name: String,
}

#[typetag::serde(name = "example")]
impl Step for Example {
    fn name(&self) -> &str {
        &self.name
    }
}
