use sea_orm::DatabaseConnection;

use crate::{
    error::AnsernoResult,
    hypertext_application_language::{Link, Resource},
};

#[async_trait::async_trait]
pub trait Model {
    fn resource_name(&self) -> &str;

    fn list_href(&self) -> String {
        format!("/{}", self.resource_name())
    }

    fn item_href(&self) -> String {
        format!("{}/{{}}", self.list_href())
    }

    fn self_href(&self, id: i32) -> String {
        format!("{}/{id}", self.list_href())
    }

    fn as_link(&self) -> AnsernoResult<Link>;

    async fn as_resource(&self, conn: &DatabaseConnection) -> AnsernoResult<Resource>;
}
