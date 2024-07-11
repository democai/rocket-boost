use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::RwLock;

static TEMPLATE_CACHE: Lazy<RwLock<Handlebars>> = Lazy::new(|| {
    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);
    RwLock::new(hb)
});

static TEMPLATE_PATH: Lazy<RwLock<PathBuf>> = Lazy::new(|| RwLock::new(PathBuf::from("templates")));

/// Loads a template from a string and registers it in the template cache.
///
/// # Arguments
///
/// * `template_name` - A string slice that holds the name of the template.
/// * `template` - A string slice that holds the template content.
///
/// # Returns
///
/// * `Result<()>` - Returns an empty result on success, or an error if the template could not be registered.
///
/// # Example
///
/// ```
/// let template_name = "example";
/// let template_content = "<html>{{content}}</html>";
/// load_from_string(template_name, template_content).await.unwrap();
/// ```
pub async fn load_from_string(template_name: &str, template: &str) -> Result<()> {
    let mut wo_cache = TEMPLATE_CACHE.write().await;
    wo_cache.register_template_string(template_name, template)?;
    Ok(())
}

// Load partials from a hashmap of name to path
// Example:
// let partials = HashMap::from([
//     ("sidebar", "content/templates/app/sidebar.html"),
//     ("navbar", "content/templates/app/navbar.html"),
// ]);
// load_partials(partials).await;
pub async fn load_partials(partials: HashMap<&str, PathBuf>) -> Result<()> {
    let mut wo_cache = TEMPLATE_CACHE.write().await;
    let template_path = TEMPLATE_PATH.read().await;
    for (name, path) in partials {
        let template = fs::read_to_string(template_path.join(path))?;
        wo_cache.register_partial(name, template)?;
    }
    Ok(())
}

pub async fn load_template(template_name: &str) -> Result<()> {
    // If the template is in the registry, is_in_cache = true
    {
        let ro_cache = TEMPLATE_CACHE.read().await;
        if ro_cache.has_template(template_name) {
            return Ok(());
        }
    }

    // If not, fetch it and add it to the registry
    let template_path = TEMPLATE_PATH.read().await;
    let mut wo_cache = TEMPLATE_CACHE.write().await;
    let template_path = template_path.join(format!("{}.html", template_name));
    log::debug!("Loading Template path: {:?}", template_path);
    let template = fs::read_to_string(&template_path)
        .with_context(|| format!("could not read template file: {:?}", template_path))?;
    // Register the template
    wo_cache.register_template_string(template_name, template)?;
    Ok(())
}

pub async fn get_registry() -> Handlebars<'static> {
    TEMPLATE_CACHE.read().await.clone()
}

pub fn render<T>(registry: &Handlebars, template_name: &str, key_values: &T) -> Result<String>
where
    T: Serialize,
{
    // Render template with data
    let rendered = registry
        .render(template_name, &key_values)
        .with_context(|| format!("Error rendering template: {}", template_name))?;

    Ok(rendered)
}

pub async fn set_template_path(path: PathBuf) {
    let mut wo_cache = TEMPLATE_PATH.write().await;
    *wo_cache = path;
}