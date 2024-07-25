use std::collections::HashMap;

use rocket::http::Status;

use crate::{BoostHeader, BoostedOption};

pub struct BoostedArgs
{
    pub code: Status,
    pub title: String,
    pub tree: BoostedOption,
    pub headers: HashMap<String, String>,
    pub boost_headers: Vec<BoostHeader>,
    pub main_template_name: String,
    pub main_template_args: HashMap<String, serde_json::Value>,
}

impl Default for BoostedArgs
{
    fn default() -> Self {
        Self {
            code: Status::Ok,
            title: "".to_string(),
            tree: BoostedOption::None,
            headers: HashMap::new(),
            boost_headers: Vec::new(),
            main_template_name: "main".to_string(),
            main_template_args: HashMap::new(),
        }
    }
}
