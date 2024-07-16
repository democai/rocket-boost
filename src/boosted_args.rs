use std::collections::HashMap;

use rocket::http::Status;
use tide_jsx::Render;

use crate::BoostHeader;

pub struct BoostedArgs<R>
where
    R: Render + Sized,
{
    pub code: Status,
    pub title: String,
    pub tree: Option<R>,
    pub headers: HashMap<String, String>,
    pub boost_headers: Vec<BoostHeader>,
    pub main_template_name: String,
    pub main_template_args: HashMap<String, serde_json::Value>,
}

impl<R> Default for BoostedArgs<R>
where
    R: Render + Sized,
{
    fn default() -> Self {
        Self {
            code: Status::Ok,
            title: "".to_string(),
            tree: None,
            headers: HashMap::new(),
            boost_headers: Vec::new(),
            main_template_name: "main".to_string(),
            main_template_args: HashMap::new(),
        }
    }
}
