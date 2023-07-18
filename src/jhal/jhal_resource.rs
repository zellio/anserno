// use crate::{
//     entities::{authors, books, data, series},
//     pagination::Paginator,
// };
use sea_orm::{DatabaseConnection, EntityTrait, Iterable, ModelTrait, PrimaryKeyToColumn};
// use serde::{ser::SerializeMap, Serialize, Serializer};

use serde::{ser::SerializeMap, Serialize, Serializer};
use std::collections::HashMap;

use crate::{
    entities::{authors, books, data, series},
    error::{AnsernoError, AnsernoResult},
    jhal::{JhalLink, JhalLinkBuilder},
    pagination::Paginator,
};

use super::JhalModel;

#[derive(Builder, Clone, Debug, Serialize)]
#[builder(build_fn(error = "AnsernoError"))]
pub struct JhalResource {
    #[serde(
        rename = "_links",
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "JhalResource::serialize_map_squish_values"
    )]
    #[builder(
        setter(into, each(name = "link", into)),
        default = "HashMap::default()"
    )]
    pub links: HashMap<String, Vec<JhalLink>>,

    #[serde(
        rename = "_embedded",
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "JhalResource::serialize_map_squish_values"
    )]
    #[builder(
        setter(into, each(name = "embed", into)),
        default = "HashMap::default()"
    )]
    pub embedded: HashMap<String, Vec<JhalResource>>,

    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    #[builder(setter(into), default = "HashMap::default()")]
    pub data: HashMap<String, serde_json::Value>,
}

impl JhalResource {
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
}

impl JhalResourceBuilder {
    pub fn from_model<E>(value: &<E as sea_orm::EntityTrait>::Model) -> Self
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: ModelTrait + Serialize + JhalModel,
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

        let mut builder = JhalResourceBuilder::default();

        builder
            .link((
                "self".to_string(),
                vec![JhalLinkBuilder::default()
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

impl JhalResource {
    pub fn from_model<E>(value: &<E as sea_orm::EntityTrait>::Model) -> Self
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: ModelTrait + Serialize + JhalModel,
        <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
    {
        JhalResourceBuilder::from_model::<E>(value).build().unwrap()
    }

    pub fn from_paginated_models<E, P>(
        models: Vec<<E as EntityTrait>::Model>,
        paginator: P,
    ) -> AnsernoResult<JhalResource>
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: ModelTrait + Serialize + JhalModel,
        <E as EntityTrait>::PrimaryKey: PrimaryKeyToColumn,
        P: Paginator,
    {
        let model = models.first().unwrap();

        let page = paginator.page();
        let pages = paginator.pages();

        let mut builder = JhalResourceBuilder::default();

        builder.link((
            "self".to_string(),
            vec![JhalLinkBuilder::default()
                .href(format!("{}?page={}", model.list_href(), page))
                .build()?],
        ));

        if page < pages {
            builder.link((
                "next".to_string(),
                vec![JhalLinkBuilder::default()
                    .href(format!("{}?page={}", model.list_href(), page + 1))
                    .build()?],
            ));
        }

        if page > 1 {
            builder.link((
                "prev".to_string(),
                vec![JhalLinkBuilder::default()
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
                    .map(|x| JhalResource::from_model::<E>(&x))
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

    pub async fn from_author(
        conn: &DatabaseConnection,
        value: &authors::Model,
    ) -> AnsernoResult<Self> {
        JhalResourceBuilder::from_model::<authors::Entity>(value)
            .link((
                "books".to_string(),
                value
                    .find_linked(authors::AuthorsToBooks)
                    .all(conn)
                    .await?
                    .iter()
                    .map(|book| {
                        JhalLinkBuilder::default()
                            .title(&book.title)
                            .href(&book.self_href(book.id))
                            .build()
                            .unwrap()
                    })
                    .collect::<Vec<JhalLink>>(),
            ))
            .build()
    }

    pub async fn from_book(conn: &DatabaseConnection, value: &books::Model) -> AnsernoResult<Self> {
        let mut builder = JhalResourceBuilder::from_model::<books::Entity>(value);

        if value.has_cover.unwrap_or(false) {
            builder
                .link((
                    "cover".to_string(),
                    vec![JhalLinkBuilder::default()
                        .href(format!("{}/cover", value.self_href(value.id)))
                        .build()?],
                ))
                .link((
                    "thumbnail".to_string(),
                    vec![JhalLinkBuilder::default()
                        .href(format!("{}/thumb", value.self_href(value.id)))
                        .build()?],
                ));
        }

        if let Ok(formats) = value.find_related(data::Entity).all(conn).await {
            builder.link((
                "downloads".to_string(),
                formats
                    .iter()
                    .map(|format| {
                        JhalLinkBuilder::default()
                            .href(format!(
                                "{}/download/{}",
                                value.self_href(value.id),
                                format.format
                            ))
                            .name(format.format.clone())
                            .build()
                            .unwrap()
                    })
                    .collect::<Vec<JhalLink>>(),
            ));
        }

        if let Ok(authors) = value.find_linked(books::BooksToAuthors).all(conn).await {
            builder.link((
                "authors".to_string(),
                authors
                    .iter()
                    .map(|author| {
                        JhalLinkBuilder::default()
                            .title(&author.name)
                            .href(author.self_href(author.id).to_string())
                            .build()
                            .unwrap()
                    })
                    .collect::<Vec<JhalLink>>(),
            ));
        }

        if let Ok(series) = value.find_linked(books::BooksToSeries).all(conn).await {
            builder.link((
                "series".to_string(),
                series
                    .iter()
                    .map(|series| {
                        JhalLinkBuilder::default()
                            .title(&series.name)
                            .href(series.self_href(series.id).to_string())
                            .build()
                            .unwrap()
                    })
                    .collect::<Vec<JhalLink>>(),
            ));
        }

        builder.build()
    }

    pub async fn from_series(
        conn: &DatabaseConnection,
        value: &series::Model,
    ) -> AnsernoResult<Self> {
        JhalResourceBuilder::from_model::<series::Entity>(value)
            .link((
                "books".to_string(),
                value
                    .find_linked(series::SeriesToBooks)
                    .all(conn)
                    .await?
                    .iter()
                    .map(|book| {
                        JhalLinkBuilder::default()
                            .title(&book.title)
                            .href(&book.self_href(book.id))
                            .build()
                            .unwrap()
                    })
                    .collect::<Vec<JhalLink>>(),
            ))
            .build()
    }
}
