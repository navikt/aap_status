#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod github;
mod table;

pub use app::TemplateApp;
pub use table::Table;
pub use github::{GitHubApi, PullRequest, Repository};
