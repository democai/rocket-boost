# rocket-boost
A Rust crate to provide allow for HTMX boosted output with handlebars fallback

```rust
use anyhow::Result;
use rocket_boost::{Boosted, BoostedArgs, jsx::{rsx, Render}};

#[get("/")]
async fn index() -> Result<Boosted<impl Render>> {
    let tree = rsx! {
        <>
            <h1>Hello, World!</h1>
            <button hx-get="/clicked" hx-target="this">Click me</button>
        </>
    };

    Boosted::try_new(
        BoostedArgs {
            title: "Example",
            tree,
            ..Default::default()
        }
    )
    .await
}
```