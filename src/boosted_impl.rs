use std::collections::HashMap;

use anyhow::Context;

use super::{
    boosted::Boosted,
    hb::{get_registry, load_template},
};
use crate::{BoostHeader, BoostedArgs, RedirectType, Result, BoostedOption};

impl Boosted
{
    pub async fn try_new(args: BoostedArgs) -> Result<Self> {
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
    /// Create a function that returns Result<Boosted<BoostedOption<impl Render + 'r>>>
    /// ```
    /// let redirect = Boosted::redirect(RedirectType::Temporary, "/example".to_string(), None).await;
    /// return redirect;
    /// ```
    /// If you need to return something else sometimes, use `tree: Some(BoostedOption::Render(tree)),`
    /// or `to_boosted_option()` on your `Boosted` instance, but that requires a Clone
    /// 
    /// 
    pub async fn redirect(
        redirect_type: RedirectType,
        url: String,
        opt_boost_headers: Option<Vec<BoostHeader>>,
    ) -> Result<Boosted> {
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
        let redirect = Boosted::try_new(BoostedArgs {
            code,
            title: "".to_string(),
            headers: HashMap::from([("Location".to_string(), url_str)]),
            boost_headers,
            tree: BoostedOption::Redirect,
            ..Default::default()
        })
        .await?;
        Ok(redirect)
    }
}
