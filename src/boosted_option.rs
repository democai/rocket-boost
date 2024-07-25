use tide_jsx::Render;

#[derive(Clone)]
pub enum BoostedOption<B>
where
    B: Render + Sized,
{
    None,
    Redirect,
    Render(B),
}

impl<B> Render for BoostedOption<B>
where
    B: Render + Sized,
{
    fn render(self) -> String {
        match self {
            BoostedOption::Render(b) => b.render(),
            _ => "".to_string(),
        }
    }

    fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> Result<(), std::fmt::Error> {
        match self {
            BoostedOption::Render(b) => b.render_into(writer),
            _ => Ok(()),
        }
    }
}
