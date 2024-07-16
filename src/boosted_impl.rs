use std::collections::HashMap;

use anyhow::Context;
use tide_jsx::Render;

use super::{
    boosted::Boosted,
    hb::{get_registry, load_template},
};
use crate::{BoostHeader, BoostedArgs, Result};

impl<R> Boosted<R>
where
    R: Render + Sized,
{
    pub async fn try_new(args: BoostedArgs<R>) -> Result<Self> {
        load_template(&args.main_template_name)
            .await
            .with_context(|| {
                format!("could not load main template: {}", args.main_template_name)
            })?;
        let registry = get_registry().await;

        Ok(Self {
            registry,
            code: args.code,
            title: args.title,
            tree: args.tree,
            headers: args.headers,
            boost_headers: args.boost_headers,
            main_template_name: args.main_template_name,
            main_template_args: args.main_template_args,
        })
    }

    pub async fn redirect(
        url: &str,
        opt_boost_headers: Option<Vec<BoostHeader>>,
    ) -> Result<Boosted<impl Render + Sized>> {
        let url_str = url.to_string();
        let mut boost_headers = vec![BoostHeader::Location(url_str.clone())];
        if let Some(boost_header_vec) = opt_boost_headers {
            for boost_header in boost_header_vec {
                boost_headers.push(boost_header);
            }
        }
        Boosted::try_new(BoostedArgs::<Option<&str>> {
            title: "".to_string(),
            headers: HashMap::from([("Location".to_string(), url_str)]),
            boost_headers,
            tree: None,
            ..Default::default()
        })
        .await
    }
}
