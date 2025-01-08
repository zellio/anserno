use serde::ser::SerializeMap;
use serde_json::Value;
use std::collections::BTreeMap;

use crate::link::Link;

fn serialize_map_squish_values<K, V, S>(
    map: &BTreeMap<K, Vec<V>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    K: serde::Serialize,
    V: serde::Serialize,
    S: serde::Serializer,
{
    let mut serialized_map = serializer.serialize_map(Some(map.len()))?;

    for (key, value) in map {
        if value.len() == 1 {
            serialized_map.serialize_entry(&key, &value.iter().next().unwrap())?;
        } else {
            serialized_map.serialize_entry(&key, &value)?;
        }
    }

    serialized_map.end()
}

#[derive(
    ::std::clone::Clone,
    ::std::default::Default,
    ::std::fmt::Debug,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Resource {
    #[serde(
        rename = "_links",
        skip_serializing_if = "BTreeMap::is_empty",
        serialize_with = "serialize_map_squish_values"
    )]
    pub links: BTreeMap<String, Vec<crate::link::Link>>,

    #[serde(
        rename = "_embedded",
        skip_serializing_if = "BTreeMap::is_empty",
        serialize_with = "serialize_map_squish_values"
    )]
    pub embedded: BTreeMap<String, Vec<Resource>>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, Value>,
}

impl Resource {
    /// Create a new empty resource. Wrapper for `Self::default()`.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds link entity to the `name` group.
    ///
    /// ```
    /// # use hypertext_application_language::{link::Link, resource::Resource};
    ///
    /// let resource = Resource::default()
    ///     .with_link("group", Link::new("http://example.com"));
    ///
    /// assert!(resource.links.contains_key("group"));
    /// assert_eq!(resource.links["group"][0].href, "http://example.com");
    /// ```
    pub fn with_link(self, name: impl Into<String>, link: impl Into<Link>) -> Self {
        let mut resource = self;

        resource
            .links
            .entry(name.into())
            .or_default()
            .push(link.into());

        resource
    }

    /// Sets the links of the `name` group.
    ///
    /// NB. This will remove all previous links.
    ///
    /// ```
    /// # use hypertext_application_language::{link::Link, resource::Resource};
    ///
    /// let resource = Resource::default()
    ///     .with_link("group", Link::new("http://example.com"))
    ///     .with_links("group", vec![Link::new("http://not-example.com")]);
    ///
    /// assert!(resource.links.contains_key("group"));
    /// assert_eq!(resource.links["group"][0].href, "http://not-example.com");
    /// ```
    pub fn with_links(
        self,
        name: impl Into<String>,
        links: impl IntoIterator<Item = impl Into<Link>>,
    ) -> Self {
        let mut resource = self;
        resource.links.insert(
            name.into(),
            links.into_iter().map(::std::convert::Into::into).collect(),
        );
        resource
    }

    /// Embedds a resource into the `name` group within the resource.
    ///
    /// ```
    /// # use hypertext_application_language::resource::Resource;
    ///
    /// let resource = Resource::default()
    ///     .with_embedded("group", Resource::default());
    ///
    /// assert!(resource.embedded.contains_key("group"));
    /// ```
    pub fn with_embedded(self, name: impl Into<String>, embedded: impl Into<Resource>) -> Self {
        let mut resource = self;

        resource
            .embedded
            .entry(name.into())
            .or_default()
            .push(::std::convert::Into::into(embedded));

        resource
    }

    /// Embedds the resources under `name` within the resource.
    ///
    /// NB. This will remove all previous resources.
    ///
    /// ```
    /// # use hypertext_application_language::resource::Resource;
    ///
    /// let resource = Resource::default()
    ///     .with_embedded("group", Resource::default())
    ///     .with_embeddeds("group", vec![Resource::default()]);
    ///
    /// assert_eq!(resource.embedded.len(), 1);
    /// ```
    pub fn with_embeddeds(
        self,
        name: impl Into<String>,
        embedded: impl IntoIterator<Item = impl Into<Resource>>,
    ) -> Self {
        let mut resource = self;

        resource.embedded.insert(
            name.into(),
            embedded
                .into_iter()
                .map(::std::convert::Into::into)
                .collect(),
        );

        resource
    }

    /// Adds a property to the resource.
    ///
    /// Example:
    ///
    /// ```
    /// # use hypertext_application_language::resource::Resource;
    ///
    /// let resource = Resource::default().with_property("field", "value");
    ///
    /// assert_eq!(resource.properties["field"], serde_json::Value::from("value"));
    /// ```
    pub fn with_property(self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        let mut resource = self;
        resource.properties.insert(name.into(), value.into());
        resource
    }

    /// Set all properties of the resource.
    ///
    /// Example:
    ///
    /// ```
    /// # use hypertext_application_language::resource::Resource;
    /// # use std::collections::BTreeMap;
    /// use serde_json::Value;
    ///
    /// let resource = Resource::default()
    ///     .with_property("field", "value")
    ///     .with_properties(BTreeMap::from([
    ///         ("new_field".to_string(), Value::from("value"))
    ///     ]));
    ///
    /// assert_eq!(resource.properties.get("field"), None);
    /// assert_eq!(resource.properties["new_field"], Value::from("value"));
    /// ```
    pub fn with_properties(self, properties: BTreeMap<String, Value>) -> Self {
        let mut resource = self;
        resource.properties = properties;
        resource
    }
}

/// Utility trait to enable `From` conversions for Type into `Resource`.
pub trait AsResource {
    fn as_resource(&self) -> Resource;
}

impl<T> ::std::convert::From<T> for Resource
where
    T: AsResource,
{
    fn from(value: T) -> Self {
        value.as_resource()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_map_squish_values() {
        let resource = Resource::default().with_link("", Link::new("http://example.com/"));

        assert_eq!(
            r#"{"_links":{"":{"href":"http://example.com/"}}}"#,
            serde_json::to_string(&resource).unwrap()
        );

        let resource = Resource::default().with_links(
            "example",
            [
                Link::new("http://example.com/"),
                Link::new("http://example.com/"),
            ],
        );

        assert_eq!(
            r#"{"_links":{"example":[{"href":"http://example.com/"},{"href":"http://example.com/"}]}}"#,
            serde_json::to_string(&resource).unwrap()
        );
    }
}
