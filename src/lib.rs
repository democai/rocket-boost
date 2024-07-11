mod boosted_impl;
mod boosted_responder;
pub mod hb;

mod boosted;
pub use boosted::*;

mod boosted_args;
pub use boosted_args::*;

mod boost_header;
pub use boost_header::*;

mod main_template_args;
pub(crate) use main_template_args::*;

// Reexport jsx
pub use tide_jsx as jsx;

pub type Result<T> = std::result::Result<T, rocket::response::Debug<anyhow::Error>>;