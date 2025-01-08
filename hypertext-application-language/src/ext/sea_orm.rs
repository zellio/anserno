use std::collections::BTreeMap;

use sea_orm::{EntityTrait, Iden, Iterable, ModelTrait};

use crate::{error::Result, link::Link, resource::Resource};

pub fn serialize_sea_orm_value(value: sea_orm::Value) -> serde_json::Result<serde_json::Value> {
    match value {
        sea_orm::Value::Bool(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::TinyInt(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::SmallInt(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::Int(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::BigInt(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::TinyUnsigned(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::SmallUnsigned(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::Unsigned(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::BigUnsigned(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::Float(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::Double(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::String(Some(value)) => serde_json::to_value(value),
        sea_orm::Value::Char(Some(value)) => serde_json::to_value(value),
        _ => Ok(serde_json::Value::Null),
    }
}

pub trait AsResource {
    fn resource_kind(&self) -> &str;

    fn resource_identifier(&self) -> impl ::std::fmt::Display;

    fn list_link(&self, page: impl ::std::fmt::Display, items: impl ::std::fmt::Display) -> Link {
        Link::new(format!(
            "{}?page={}&items={}",
            self.resource_kind(),
            page,
            items
        ))
    }

    #[inline]
    fn item_link(&self) -> Link {
        Link::new(format!("/{}/{{}}", self.resource_kind()))
    }

    #[inline]
    fn self_link(&self) -> Link {
        Link::new(format!(
            "/{}/{}",
            self.resource_kind(),
            self.resource_identifier()
        ))
    }

    #[inline]
    fn self_entity_link(&self, entity: impl ::std::fmt::Display) -> Link {
        Link::new(format!(
            "/{}/{}/{}",
            self.resource_kind(),
            self.resource_identifier(),
            entity,
        ))
    }

    fn as_resource(
        &self,
        conn: &sea_orm::DatabaseConnection,
    ) -> impl std::future::Future<Output = Result<Resource>>;
}

impl crate::resource::Resource {
    pub fn from_model<E>(value: &<E as EntityTrait>::Model) -> Result<Self>
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: ModelTrait + AsResource,
    {
        let mut properties = BTreeMap::default();

        for column in E::Column::iter() {
            properties.insert(
                column.to_string(),
                serialize_sea_orm_value(value.get(column))?,
            );
        }

        Ok(Resource::default()
            .with_link("self", value.self_link())
            .with_properties(properties))
    }
}
