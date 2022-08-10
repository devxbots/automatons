use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::name;

name!(
    /// Check run output title
    CheckRunOutputTitle
);

name!(
    /// Check out output summary
    CheckRunOutputSummary
);

/// Check run output
///
/// Integrations can provide additional context for a completed check run in the check run's output.
/// The output has a title and a summary, which supports Markdown. It can optionally have a text
/// with more details, and annotations.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct CheckRunOutput {
    title: CheckRunOutputTitle,
    summary: CheckRunOutputSummary,
    text: Option<String>,
    annotations_count: u64,
    annotations_url: Url,
}

impl CheckRunOutput {
    /// Returns the check run output's title.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn title(&self) -> &CheckRunOutputTitle {
        &self.title
    }

    /// Returns the check run output's summary.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn summary(&self) -> &CheckRunOutputSummary {
        &self.summary
    }

    /// Returns the check run output's text.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn text(&self) -> &Option<String> {
        &self.text
    }

    /// Returns the check run output's annotations count.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn annotations_count(&self) -> u64 {
        self.annotations_count
    }

    /// Returns the API endpoint to query the check run output's annotations.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn annotations_url(&self) -> &Url {
        &self.annotations_url
    }
}

impl Display for CheckRunOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::{CheckRunOutput, CheckRunOutputSummary, CheckRunOutputTitle};

    const JSON: &str = r#"
    {
        "title": "2/2 checks succeeded",
        "summary": "... long Markdown content ...",
        "text": null,
        "annotations_count": 0,
        "annotations_url": "https://api.github.com/repos/devxbots/automatons/check-runs/7668626402/annotations"
    }
    "#;

    #[test]
    fn trait_deserialize() {
        let output: CheckRunOutput = serde_json::from_str(JSON).unwrap();

        assert_eq!("2/2 checks succeeded", output.title().get());
    }

    #[test]
    fn trait_display() {
        let output = CheckRunOutput {
            title: CheckRunOutputTitle::new("2/2 checks succeeded"),
            summary: CheckRunOutputSummary::new("... long Markdown content ..."),
            text: None,
            annotations_count: 0,
            annotations_url: Url::parse("https://api.github.com/repos/devxbots/automatons/check-runs/7669942377/annotations").unwrap()
        };

        assert_eq!("2/2 checks succeeded", output.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunOutput>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunOutput>();
    }
}
