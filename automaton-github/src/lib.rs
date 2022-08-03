//! GitHub Integration for the Automatons Platform
//!
//! The [automatons] platform is an automation framework for software developers, and this crate
//! enables users to interact with GitHub. It defines _resources_ that closely match the resources
//! in [GitHub's REST API](https://docs.github.com/en/rest), _tasks_ that interact with these
//! resources, and _events_ that can be consumed by (and trigger) automations.
//!
//! [automatons]: https://github.com/devxbots/automatons

#![deny(missing_docs)]

mod macros;

pub mod resource;
