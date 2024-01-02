//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "anserno_flat_books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub title: String,
    pub sort: String,
    pub path: String,
    pub authors: serde_json::Value,
    pub series: serde_json::Value,
    #[sea_orm(column_type = "Double", nullable)]
    pub series_index: f64,
    pub formats: serde_json::Value,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::authors::Entity> for Entity {
    fn to() -> RelationDef {
        super::books_authors_link::Relation::Author.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::books_authors_link::Relation::FlatBook.def().rev())
    }
}

impl Related<super::series::Entity> for Entity {
    fn to() -> RelationDef {
        super::books_series_link::Relation::Series.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::books_series_link::Relation::FlatBook.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}