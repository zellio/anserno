//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::{
    error::AnsernoResult,
    hypertext_application_language::{Link, LinkBuilder, Resource, ResourceBuilder},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "authors")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub name: String,
    pub sort: Option<String>,
    pub link: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books_authors_link::Entity")]
    BooksAuthorsLink,
}

impl Related<super::books_authors_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BooksAuthorsLink.def()
    }
}

impl Related<super::flat_books::Entity> for Entity {
    fn to() -> RelationDef {
        super::books_authors_link::Relation::FlatBook.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::books_authors_link::Relation::Author.def().rev())
    }
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        super::books_authors_link::Relation::Book.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::books_authors_link::Relation::Author.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[async_trait::async_trait]
impl crate::hypertext_application_language::Model for Model {
    fn resource_name(&self) -> &str {
        "authors"
    }

    fn as_link(&self) -> AnsernoResult<Link> {
        LinkBuilder::default()
            .title(&self.name)
            .href(&self.self_href(self.id))
            .build()
    }

    async fn as_resource(&self, conn: &DatabaseConnection) -> AnsernoResult<Resource> {
        ResourceBuilder::from_model::<Entity>(self)
            .link((
                "books".to_string(),
                self.find_related(super::books::Entity)
                    .all(conn)
                    .await?
                    .iter()
                    .map(|book| book.as_link())
                    .collect::<Result<Vec<_>, _>>()?,
            ))
            .build()
    }
}
