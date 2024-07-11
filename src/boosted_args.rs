use rocket::http::Status;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use tide_jsx::Render;

use crate::BoostHeader;

pub struct BoostedArgs<R, M>
where
    R: Render + Sized,
    M: Serialize + DeserializeOwned + Clone,
{
    pub code: Status,
    pub title: String,
    pub tree: Option<R>,
    pub headers: HashMap<String, String>,
    pub boost_headers: Vec<BoostHeader>,
    pub main_template_name: String,
    pub main_template_args: HashMap<String, M>,
}

impl<R, M> Default for BoostedArgs<R, M>
where
    R: Render + Sized,
    M: Serialize + DeserializeOwned + Clone + Default,
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
