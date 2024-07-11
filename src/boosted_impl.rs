use anyhow::{Context, Result};
use serde::Serialize;
use tide_jsx::Render;

use crate::BoostedArgs;

use super::{
    boosted::Boosted,
    hb::{get_registry, load_template},
};
use serde::de::DeserializeOwned;

impl<R, M> Boosted<R, M>
where
    R: Render + Sized,
    M: Serialize + DeserializeOwned + Clone,
{
    pub async fn try_new(
        args: BoostedArgs<R, M>,
    ) -> Result<Self> {
        load_template(&args.main_template_name)
            .await
            .with_context(|| format!("could not load main template: {}", args.main_template_name))?;
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
}
