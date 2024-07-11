# rocket-boost
A Rust crate to provide allow for HTMX boosted output with handlebars fallback

```rust
use anyhow::Result;
use rocket_boost::{Boosted, BoostedArgs, jsx::{rsx, Render}, hb::{set_template_path, load_template, load_partials}};
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<Boosted<impl Render>> {
    let tree = rsx! {
        <>
            <h1>Hello, World!</h1>
            <button hx-get="/clicked" hx-target="this">Click me</button>
        </>
    };

    set_template_path(PathBuf::from("./templates")).await;

    // load template/main.html as a handlebars template
    load_template("main").await?;

    // load partials from templates/app/sidebar.html and templates/app/navbar.html
    load_partials(HashMap::from([
        ("sidebar", "app/sidebar.html"),
        ("navbar", "app/navbar.html"),
    ]);).await?

    load_from_string("helloworld", "<h4>{{title}}</h4>{{body}}").await?;
    load_from_string("atbuildtime", include_str!("./templates/atbuildtime.html")).await?;

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