use std::collections::HashMap;

use handlebars::Handlebars;
use rocket::http::Status;
use serde::{de::DeserializeOwned, Serialize};
use tide_jsx::Render;

use crate::BoostHeader;

pub struct Boosted<R, M>
where
    R: Render + Sized,
    M: Serialize + DeserializeOwned + Clone,
{
    pub(super) registry: Handlebars<'static>,
    pub(super) code: Status,
    pub(super) title: String,
    pub(super) tree: R,
    pub(super) headers: HashMap<String, String>,
    pub(super) boost_headers: Vec<BoostHeader>,
    pub(super) main_template_name: String,
    pub(super) main_template_args: M,
}
