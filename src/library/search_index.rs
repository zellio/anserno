use sea_orm::ConnectionTrait;

use crate::{app::Context, error::AnsernoResult};

#[derive(Debug, Clone, Default)]
pub struct SearchIndex {}

impl SearchIndex {
    async fn execute(
        &self,
        ctx: &Context,
        statement: sea_orm::Statement,
    ) -> AnsernoResult<sea_orm::ExecResult> {
        Ok(ctx.database_connection.execute(statement).await?)
    }

    pub fn drop_search_index_statement(&self) -> sea_orm::Statement {
        sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"DROP TABLE IF EXISTS "anserno_search_index""#.to_string(),
        )
    }

    pub async fn drop_search_index(&self, ctx: &Context) -> AnsernoResult<sea_orm::ExecResult> {
        self.execute(ctx, self.drop_search_index_statement()).await
    }

    pub fn create_search_index_statement(&self) -> sea_orm::Statement {
        sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            concat!(
                r#"CREATE VIRTUAL TABLE IF NOT EXISTS "anserno_search_index" USING fts5 ("#,
                r#"  title, sort, authors, series, formats, description"#,
                r#")"#,
            )
            .to_string(),
        )
    }

    pub async fn create_search_index(&self, ctx: &Context) -> AnsernoResult<sea_orm::ExecResult> {
        self.execute(ctx, self.create_search_index_statement())
            .await
    }

    pub fn insert_search_index_statement(&self) -> sea_orm::Statement {
        sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            concat!(
                r#"INSERT INTO "anserno_search_index" ("#,
                r#"  rowid, title, sort, authors, series, formats, description"#,
                r#")"#,
                r#"SELECT"#,
                r#"    "books"."id" AS "rowid","#,
                r#"    "books"."title" AS "title","#,
                r#"    "books"."sort" AS "sort","#,
                r#"    RTRIM(REPLACE(GROUP_CONCAT(DISTINCT "authors"."name" || '@'), '@,', ', '), '@') AS "authors","#,
                r#"    RTRIM(REPLACE(GROUP_CONCAT(DISTINCT "series"."name" || '@'), '@,', ', '), '@') AS "series","#,
                r#"    RTRIM(REPLACE(GROUP_CONCAT(DISTINCT "data"."format" || '@'), '@,', ', '), '@') AS "formats","#,
                r#"    "comments"."text" AS "description""#,
                r#"FROM"#,
                r#"    "books""#,
                r#"    LEFT JOIN "data" ON "books"."id" = "data"."book""#,
                r#"    LEFT JOIN "comments" ON "books"."id" = "comments"."book""#,
                r#"    LEFT JOIN "books_authors_link" ON "books"."id" = "books_authors_link"."book""#,
                r#"    LEFT JOIN "authors" ON "authors"."id" = "books_authors_link"."author""#,
                r#"    LEFT JOIN "books_series_link" ON "books"."id" = "books_series_link"."book""#,
                r#"    LEFT JOIN "series" ON "series"."id" = "books_series_link"."series""#,
                r#"GROUP BY"#,
                r#"    "books"."id""#,
            ).to_string(),
        )
    }

    pub async fn insert_search_index(&self, ctx: &Context) -> AnsernoResult<sea_orm::ExecResult> {
        self.execute(ctx, self.insert_search_index_statement())
            .await
    }
}
