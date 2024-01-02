use crate::error::AnsernoError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Deserialize, Serialize)]
#[builder(build_fn(error = "AnsernoError"), setter(into, strip_option))]
pub struct Link {
    #[builder(setter(into))]
    pub href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub templated: Option<bool>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub deprecation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub hreflang: Option<String>,
}
