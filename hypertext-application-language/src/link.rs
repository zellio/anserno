use serde_json::Value;
use std::collections::BTreeMap;

#[derive(serde::Deserialize, serde::Serialize, ::std::clone::Clone, ::std::fmt::Debug)]
pub struct Link {
    pub href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated: Option<bool>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, Value>,
}

impl Link {
    /// Creates a link with the given href.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    /// let link = Link::new("http://example.com");
    /// ```
    pub fn new(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            templated: None,
            kind: None,
            deprecation: None,
            name: None,
            profile: None,
            title: None,
            hreflang: None,
            properties: BTreeMap::default(),
        }
    }

    /// Sets the templated flag of the link.
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com");
    /// assert_eq!(link.templated, None);
    ///
    /// let link = Link::new("http://example.com").with_templated(true);
    /// assert_eq!(link.templated, Some(true));
    /// ```
    pub fn with_templated(self, templated: impl Into<bool>) -> Self {
        let mut link = self;
        link.templated = Some(templated.into());
        link
    }

    /// Sets the type field of the link.
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com");
    /// assert_eq!(link.kind, None);
    ///
    /// let link = Link::new("http://example.com").with_type("type");
    /// assert_eq!(link.kind, Some("type".to_string()));
    /// ```
    pub fn with_type(self, kind: impl Into<String>) -> Self {
        let mut link = self;
        link.kind = Some(kind.into());
        link
    }

    /// Sets the deprecation message of the link.
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com");
    /// assert_eq!(link.deprecation, None);
    ///
    /// let link = Link::new("http://example.com").with_deprecation("Reason deprecated");
    /// assert_eq!(link.deprecation, Some("Reason deprecated".to_string()));
    /// ```
    pub fn with_deprecation(self, deprecation: impl Into<String>) -> Self {
        let mut link = self;
        link.deprecation = Some(deprecation.into());
        link
    }

    /// Sets the name of the link.
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com");
    /// assert_eq!(link.name, None);
    ///
    /// let link = Link::new("http://example.com").with_name("name");
    /// assert_eq!(link.name, Some("name".to_string()));
    /// ```
    pub fn with_name(self, name: impl Into<String>) -> Self {
        let mut link = self;
        link.name = Some(name.into());
        link
    }

    /// Sets the profile of the link.
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com");
    /// assert_eq!(link.profile, None);
    ///
    /// let link = Link::new("http://example.com").with_profile("profile");
    /// assert_eq!(link.profile, Some("profile".to_string()));
    /// ```
    pub fn with_profile(self, profile: impl Into<String>) -> Self {
        let mut link = self;
        link.profile = Some(profile.into());
        link
    }

    /// Sets the title of the link.
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com");
    /// assert_eq!(link.title, None);
    ///
    /// let link = Link::new("http://example.com").with_title("title");
    /// assert_eq!(link.title, Some("title".to_string()));
    /// ```
    pub fn with_title(self, title: impl Into<String>) -> Self {
        let mut link = self;
        link.title = Some(title.into());
        link
    }

    /// Sets the href language of the link.
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com");
    /// assert_eq!(link.hreflang, None);
    ///
    /// let link = Link::new("http://example.com").with_hreflang("lang");
    /// assert_eq!(link.hreflang, Some("lang".to_string()));
    /// ```
    pub fn with_hreflang(self, hreflang: impl Into<String>) -> Self {
        let mut link = self;
        link.hreflang = Some(hreflang.into());
        link
    }

    /// Adds a property to the link.
    ///
    /// Example:
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    ///
    /// let link = Link::new("http://example.com").with_property("field", "value");
    ///
    /// assert_eq!(link.properties["field"], serde_json::Value::from("value"));
    /// ```
    pub fn with_property(self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        let mut link = self;
        link.properties.insert(name.into(), value.into());
        link
    }

    /// Set all properties of the link.
    ///
    /// Example:
    ///
    /// ```
    /// # use hypertext_application_language::link::Link;
    /// # use std::collections::BTreeMap;
    /// use serde_json::Value;
    ///
    /// let link = Link::new("http://example.com")
    ///     .with_property("field", "value")
    ///     .with_properties(BTreeMap::from([
    ///         ("new_field".to_string(), Value::from("value"))
    ///     ]));
    ///
    /// assert_eq!(link.properties.get("field"), None);
    /// assert_eq!(link.properties["new_field"], Value::from("value"));
    /// ```
    pub fn with_properties(self, properties: BTreeMap<String, Value>) -> Self {
        let mut link = self;
        link.properties = properties;
        link
    }
}

/// Utility trait to enable `From` conversions for Type into `Link`.
///
/// Example:
///
/// ```
/// # use hypertext_application_language::link::{Link, AsLink};
///
/// struct Entity {
///     name: String,
///     title: String,
/// }
///
/// impl AsLink for Entity {
///     fn as_link(&self) -> Link {
///         Link::new(format!("http://example.com/{}", self.name)).with_title(&self.title)
///     }
/// }
///
/// let entity = Entity { name: "name".to_string(), title: "title".to_string() };
/// let link = Link::from(entity);
///
/// assert_eq!(link.title, Some("title".to_string()));
/// ```
pub trait AsLink {
    /// Convert type into a HAL link
    fn as_link(&self) -> Link;
}

impl<T> ::std::convert::From<T> for Link
where
    T: AsLink,
{
    fn from(value: T) -> Self {
        value.as_link()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_href_only() {
        let link = Link::new("http://example.com/");

        assert_eq!(
            r#"{"href":"http://example.com/"}"#,
            serde_json::to_string(&link).unwrap()
        )
    }

    #[test]
    fn test_optional_fields() {
        let link = Link::new("http://example.com/")
            .with_name("Foo")
            .with_title("Bar");

        assert_eq!(
            r#"{"href":"http://example.com/","name":"Foo","title":"Bar"}"#,
            serde_json::to_string(&link).unwrap()
        )
    }

    #[test]
    fn test_properties() {
        let link = Link::new("http://example.com/")
            .with_name("Foo")
            .with_property("currentlyProcessing", 14_i32)
            .with_property("shippedToday", 14.2_f64);

        assert_eq!(
            r#"{"href":"http://example.com/","name":"Foo","currentlyProcessing":14,"shippedToday":14.2}"#,
            serde_json::to_string(&link).unwrap()
        )
    }
}
