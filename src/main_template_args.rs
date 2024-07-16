use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone)]
pub(super) struct MainTemplateArgsInternal();

impl MainTemplateArgsInternal {
    /// Creates a new `MainTemplateArgsInternal` from a title, content, and serializable data.
    ///
    /// # Arguments
    ///
    /// * `title` - A string slice that holds the title.
    /// * `content` - A string slice that holds the content.
    /// * `serializable` - The serializable data of type T.
    ///
    /// # Returns
    ///
    /// Returns a new Serializable instance that can be used in the main template.
    /// It should give back
    pub(super) fn from_serializable<T>(
        title: &str,
        content: &str,
        serializable: &HashMap<String, T>,
    ) -> Result<HashMap<String, StringOrT<T>>>
    where
        T: Serialize + Clone,
    {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut new_serializable: HashMap<String, StringOrT<T>> = HashMap::new();
        new_serializable.insert("title".to_string(), StringOrT::String(title.to_string()));
        new_serializable.insert(
            "content".to_string(),
            StringOrT::String(content.to_string()),
        );
        new_serializable.insert(
            "timestamp".to_string(),
            StringOrT::String(timestamp.to_string()),
        );
        for (key, value) in serializable.iter() {
            new_serializable.insert(key.to_string(), StringOrT::T(value.clone()));
        }
        Ok(new_serializable)
    }
}

pub(super) enum StringOrT<T: Serialize> {
    String(String),
    T(T),
}

impl<T: Serialize> Serialize for StringOrT<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrT::String(s) => s.serialize(serializer),
            StringOrT::T(t) => t.serialize(serializer),
        }
    }
}
