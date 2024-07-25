use std::collections::HashMap;

use handlebars::Handlebars;
use rocket::http::Status;

use crate::{BoostHeader, BoostedOption};

pub struct Boosted
{
    pub(super) registry: Handlebars<'static>,
    pub(super) code: Status,
    pub(super) title: String,
    pub(super) tree: BoostedOption,
    pub(super) headers: HashMap<String, String>,
    pub(super) boost_headers: Vec<BoostHeader>,
    pub(super) main_template_name: String,
    pub(super) main_template_args: HashMap<String, serde_json::Value>,
}
