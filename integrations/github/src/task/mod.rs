//! Tasks for the GitHub
//!
//! The GitHub integration implements tasks that can be used to create automatons.

pub use self::create_check_run::{CreateCheckRun, CreateCheckRunInput};
pub use self::list_check_runs_for_check_suite::ListCheckRunsForCheckSuite;
pub use self::list_check_runs_for_git_sha::ListCheckRunsForGitSha;
pub use self::list_check_suites::ListCheckSuites;

mod create_check_run;
mod list_check_runs_for_check_suite;
mod list_check_runs_for_git_sha;
mod list_check_suites;
