use std::io::Cursor;

use rocket::{
    http::{ContentType, Header, Status},
    response::Responder,
    Request, Response,
};
use serde::{de::DeserializeOwned, Serialize};
use tide_jsx::Render;

use super::boosted::Boosted;
use crate::{hb::render, MainTemplateArgsInternal};

#[rocket::async_trait]
impl<'r, R, M> Responder<'r, 'r> for Boosted<R, M>
where
    R: Render + Sized,
    M: Serialize + DeserializeOwned + Clone,
{
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let boosted = request.headers().get_one("HX-Request") == Some("true");
        match boosted {
            true => {
                let body = match self.title.is_empty() {
                    true => self.tree.render(),
                    false => format!("<title>{}</title>\n{}", self.title, self.tree.render()),
                };
                let mut response = Response::build();
                response
                    .header(ContentType::HTML)
                    .sized_body(body.len(), Cursor::new(body));
                for boost_header in &self.boost_headers {
                    let header: Header<'_> = boost_header.into();
                    response.header(header);
                }

                response.ok()
            }
            false => {

                let main_template_args_internal = MainTemplateArgsInternal::from_serializable(&self.title, &self.tree.render(), &self.main_template_args).map_err(|e| {
                    log::error!("Error creating main template args: {:?}", e);
                    Status::InternalServerError
                })?;
                let body = render(
                    &self.registry,
                    &self.main_template_name,
                    &main_template_args_internal,
                )
                .map_err(|e| {
                    log::error!("Error rendering main: {:?}", e);
                    Status::InternalServerError
                })?;

                let mut response = Response::build();
                response
                    .header(ContentType::HTML)
                    .sized_body(body.len(), Cursor::new(body))
                    .status(self.code);
                for (key, value) in &self.headers {
                    response.header(Header::new(key.clone(), value.clone()));
                }
                response.ok()
            }
        }
    }
}
