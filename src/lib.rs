#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod graphics;

pub use app::WasmUiApp;
pub use graphics::{Graphics, Screen};
