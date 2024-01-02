use derive_builder::Builder;
use sea_orm::{EntityTrait, Iterable, ModelTrait, PrimaryKeyToColumn};
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::collections::HashMap;

use crate::{
    error::{AnsernoError, AnsernoResult},
    hypertext_application_language::{Link, LinkBuilder, Model},
    pagination::Paginator,
};

#[derive(Builder, Clone, Deserialize, Serialize)]
#[builder(build_fn(error = "AnsernoError"))]
pub struct Resource {
    #[serde(
        rename = "_links",
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "Resource::serialize_map_squish_values"
    )]
    #[builder(
        setter(into, each(name = "link", into)),
        default = "HashMap::default()"
    )]
    pub links: HashMap<String, Vec<Link>>,

    #[serde(
        rename = "_embedded",
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "Resource::serialize_map_squish_values"
    )]
    #[builder(
        setter(into, each(name = "embed", into)),
        default = "HashMap::default()"
    )]
    pub embedded: HashMap<String, Vec<Resource>>,

    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    #[builder(setter(into), default = "HashMap::default()")]
    pub data: HashMap<String, serde_json::Value>,
}

impl ResourceBuilder {
    pub fn from_model<E>(value: &<E as sea_orm::EntityTrait>::Model) -> Self
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: ModelTrait + Serialize + Model,
        <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
    {
        let id = <E as EntityTrait>::PrimaryKey::iter()
            .next()
            .map(|key| {
                value.get(<<E as EntityTrait>::PrimaryKey as PrimaryKeyToColumn>::into_column(key))
            })
            .unwrap()
            .unwrap::<Option<i32>>()
            .unwrap_or_default();

        let mut builder = ResourceBuilder::default();

        builder
            .link((
                "self".to_string(),
                vec![LinkBuilder::default()
                    .href(format!("{}/{id}", value.list_href()))
                    .build()
                    .unwrap()],
            ))
            .data(HashMap::from_iter(
                serde_json::to_value(value)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned())),
            ));

        builder
    }
}

impl Resource {
    fn serialize_map_squish_values<K, V, S>(
        map: &HashMap<K, Vec<V>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        K: Serialize,
        V: Serialize,
        S: Serializer,
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

    pub fn from_model<E>(value: &<E as sea_orm::EntityTrait>::Model) -> Self
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: ModelTrait + Serialize + Model,
        <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
    {
        ResourceBuilder::from_model::<E>(value).build().unwrap()
    }

    pub fn from_paginated_models<E, P>(
        models: Vec<<E as EntityTrait>::Model>,
        paginator: P,
    ) -> AnsernoResult<Resource>
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: ModelTrait + Serialize + Model,
        <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
        P: Paginator,
    {
        let model = models.first().unwrap();

        let page = paginator.page();
        let pages = paginator.pages();

        let mut builder = ResourceBuilder::default();

        builder.link((
            "self".to_string(),
            vec![LinkBuilder::default()
                .href(format!("{}?page={}", model.list_href(), page))
                .build()?],
        ));

        if page < pages {
            builder.link((
                "next".to_string(),
                vec![LinkBuilder::default()
                    .href(format!("{}?page={}", model.list_href(), page + 1))
                    .build()?],
            ));
        }

        if page > 1 {
            builder.link((
                "prev".to_string(),
                vec![LinkBuilder::default()
                    .href(format!("{}?page={}", model.list_href(), page - 1))
                    .build()?],
            ));
        }

        builder
            .embed((
                "items".to_string(),
                models
                    .iter()
                    .map(std::borrow::ToOwned::to_owned)
                    .map(|x| Resource::from_model::<E>(&x))
                    .collect(),
            ))
            .data(HashMap::from_iter(vec![
                (
                    "page".to_string(),
                    serde_json::Number::from_f64(page as f64).into(),
                ),
                (
                    "pages".to_string(),
                    serde_json::Number::from_f64(pages as f64).into(),
                ),
                (
                    "count".to_string(),
                    serde_json::Number::from_f64(models.len() as f64).into(),
                ),
            ]))
            .build()
    }
}
