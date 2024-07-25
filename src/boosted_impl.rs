use std::collections::HashMap;

use anyhow::Context;
use tide_jsx::Render;

use super::{
    boosted::Boosted,
    hb::{get_registry, load_template},
};
use crate::{BoostHeader, BoostedArgs, RedirectType, Result, BoostedOption};

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

    /// Creates a new `Boosted` instance for a redirect response.
    ///
    /// This method sets up a redirect to the specified URL, optionally including additional Boost headers.
    ///
    /// # Arguments
    ///
    /// * `redirect_type` - The type of redirect (Temporary or Permanent).
    /// * `url` - The URL to redirect to.
    /// * `opt_boost_headers` - Optional additional Boost headers to include in the response.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Boosted` instance, or an error if creation fails.
    ///
    /// # Example
    ///
    /// ```
    /// let redirect = Boosted::<Option<String>>::redirect(RedirectType::Temporary, "/example".to_string(), None).await;
    /// return redirect;
    /// ```
    /// 
    pub async fn redirect(
        redirect_type: RedirectType,
        url: String,
        opt_boost_headers: Option<Vec<BoostHeader>>,
    ) -> Result<Boosted<BoostedOption<R>>> {
        let url_str = url.clone();
        let mut boost_headers = vec![BoostHeader::Location(url_str.clone())];
        if let Some(boost_header_vec) = opt_boost_headers {
            for boost_header in boost_header_vec {
                boost_headers.push(boost_header);
            }
        }
        let code = match redirect_type {
            RedirectType::Temporary => rocket::http::Status::Found,
            RedirectType::Permanent => rocket::http::Status::MovedPermanently,
        };
        let redirect = Boosted::try_new(BoostedArgs::<BoostedOption<R>> {
            code,
            title: "".to_string(),
            headers: HashMap::from([("Location".to_string(), url_str)]),
            boost_headers,
            tree: None,
            ..Default::default()
        })
        .await?;
        Ok(redirect)
    }

    pub fn to_boosted_option(&self) -> Result<Boosted<BoostedOption<R>>> 
    where
        R: Clone,
    {
        let new_tree = self.tree.clone().map(|tree| BoostedOption::Render(tree.clone()));
        Ok(Boosted {
            registry: self.registry.clone(),
            code: self.code,
            title: self.title.clone(),
            tree: new_tree,
            headers: self.headers.clone(),
            boost_headers: self.boost_headers.clone(),
            main_template_name: self.main_template_name.clone(),
            main_template_args: self.main_template_args.clone(),
        })
    }
}
