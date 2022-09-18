//! Tasks for the GitHub
//!
//! The GitHub integration implements tasks that can be used to create automatons.

pub use self::list_check_runs::ListCheckRunsForCheckSuite;
pub use self::list_check_suites::ListCheckSuites;

mod list_check_runs;
mod list_check_suites;
