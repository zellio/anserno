use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

use crate::error::AnsernoResult;

#[async_trait::async_trait]
pub trait AnsernoQuery {
    fn create_query_str() -> &'static str;

    fn populate_query_str() -> &'static str;

    async fn execute(conn: &DatabaseConnection) -> AnsernoResult<()> {
        conn.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            Self::create_query_str(),
        ))
        .await?;

        conn.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            Self::populate_query_str(),
        ))
        .await?;

        Ok(())
    }
}
