//! Tasks for the GitHub
//!
//! The GitHub integration implements tasks that can be used to create automatons.

use serde::Serialize;

use crate::resource::{CheckRunOutput, CheckRunOutputSummary, CheckRunOutputTitle};

pub use self::create_check_run::{CreateCheckRun, CreateCheckRunArgs};
pub use self::get_file::GetFile;
pub use self::list_check_runs_for_check_suite::ListCheckRunsForCheckSuite;
pub use self::list_check_runs_for_git_sha::ListCheckRunsForGitSha;
pub use self::list_check_suites::ListCheckSuites;
pub use self::update_check_run::{UpdateCheckRun, UpdateCheckRunArgs};

mod create_check_run;
mod get_file;
mod list_check_runs_for_check_suite;
mod list_check_runs_for_git_sha;
mod list_check_suites;
mod update_check_run;

/// Input for check run output
///
/// Check runs can accept a variety of data in the `output` object, including a `title` and
/// `summary` and can optionally provide descriptive details about the run.
///
/// https://docs.github.com/en/rest/checks/runs#update-a-check-run
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct CheckRunOutputArgs {
    /// The title of the check run output.
    pub title: CheckRunOutputTitle,

    /// The summary of the check run output.
    pub summary: CheckRunOutputSummary,

    /// The text with descriptive details about the check run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl From<&CheckRunOutput> for CheckRunOutputArgs {
    fn from(output: &CheckRunOutput) -> Self {
        Self {
            title: output.title().clone(),
            summary: output.summary().clone(),
            text: output.text().clone(),
        }
    }
}
