use std::sync::Arc;

use tide_jsx::{BoxedRender, Render};

#[derive(Clone)]
pub enum BoostedOption
{
    None,
    Redirect,
    Render(Arc<BoxedRender>),
}

impl BoostedOption {
    pub fn from_tree(tree: impl Render + 'static + Send + Sync) -> Self {
        BoostedOption::Render(Arc::new(Box::new(tree)))
    }
}



impl Render for BoostedOption
{
    fn render(&self) -> String {
        match self {
            BoostedOption::Render(b) => b.render(),
            _ => "".to_string(),
        }
    }

    fn render_into(&self, writer: &mut String) -> Result<(), std::fmt::Error> {
        match self {
            BoostedOption::Render(b) => b.render_into(writer),
            _ => Ok(()),
        }
    }
}
