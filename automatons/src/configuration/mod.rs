use serde::{Deserialize, Serialize};

mod v1;

pub use self::v1::V1;

#[derive(Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum Configuration {
    #[serde(rename = "1")]
    V1(V1),
}

#[cfg(test)]
mod tests {
    use super::Configuration;

    #[test]
    fn deserialize_v1() {
        let v1 = r#"
            version: 1
            steps:
              - example:
                  name: example
        "#;

        let configuration: Configuration = serde_yaml::from_str(v1).unwrap();
        let Configuration::V1(v1) = configuration;

        assert_eq!(1, v1.steps().len());
        assert_eq!("example", v1.steps().first().unwrap().name());
    }
}
