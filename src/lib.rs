#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod pulls;

pub use app::TemplateApp;
pub use pulls::Pulls;
