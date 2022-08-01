mod example;

#[typetag::serde]
pub trait Step {
    fn name(&self) -> &str;
}
