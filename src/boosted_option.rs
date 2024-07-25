use std::sync::Arc;

use tide_jsx::Render;

#[derive(Clone)]
pub enum BoostedOption<'a> {
    None,
    Redirect,
    Render(Arc<Box<dyn Render + 'a>>),
}

impl<'a> BoostedOption<'a> {
    pub fn from_tree(tree: impl Render + 'a + Send + Sync) -> Self {
        BoostedOption::Render(Arc::new(Box::new(tree)))
    }
}

impl<'a> Render for BoostedOption<'a> {
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
