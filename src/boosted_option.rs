use tide_jsx::Render;

#[derive(Clone)]
pub enum BoostedOption<B>
where
    B: Render + Sized,
{
    Redirect,
    Render(B),
}

impl<B> Render for BoostedOption<B>
where
    B: Render + Sized,
{
    fn render(self) -> String {
        match self {
            BoostedOption::Redirect => "".to_string(),
            BoostedOption::Render(b) => b.render(),
        }
    }

    fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> Result<(), std::fmt::Error> {
        match self {
            BoostedOption::Redirect => "".render_into(writer),
            BoostedOption::Render(b) => b.render_into(writer),
        }
    }
}
