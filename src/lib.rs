#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod pulls;
mod table;

pub use app::TemplateApp;
pub use pulls::Pulls;
pub use table::Table;
