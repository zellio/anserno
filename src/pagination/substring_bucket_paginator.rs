use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select};
use serde::{Deserialize, Serialize};

use crate::{
    error::{AnsernoError, AnsernoResult},
    pagination::{PaginationUrlQueryParams, Paginator, PaginatorDisplayConfig},
    queries::paginator::substring_buckets,
};

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(build_fn(error = "AnsernoError"))]
pub struct SubstringBucketPaginator<C: ColumnTrait> {
    #[serde(skip_serializing)]
    pub bucket_column: C,

    pub buckets: Vec<String>,

    #[builder(default = "0usize")]
    pub offset: usize,

    #[builder(default = "PaginatorDisplayConfig::default()")]
    pub display_config: PaginatorDisplayConfig,
}

impl<C> Paginator for SubstringBucketPaginator<C>
where
    C: ColumnTrait,
{
    fn display_config(&self) -> PaginatorDisplayConfig {
        self.display_config
    }

    fn page(&self) -> usize {
        self.offset + 1
    }

    fn pages(&self) -> usize {
        self.buckets.len()
    }

    fn selector<E: EntityTrait>(&self, query: Select<E>) -> Select<E> {
        query.filter(
            self.bucket_column
                .starts_with(self.buckets.get(self.offset).unwrap()),
        )
    }
}

impl<C> SubstringBucketPaginator<C>
where
    C: ColumnTrait,
{
    pub async fn buckets<E>(
        conn: &DatabaseConnection,
        bucket_column: C,
        substring_length: usize,
    ) -> AnsernoResult<Vec<String>>
    where
        E: EntityTrait,
    {
        substring_buckets::<E, C, String>(bucket_column, substring_length)
            .all(conn)
            .await
            .map_err(AnsernoError::from)
    }
}

impl<C> From<PaginationUrlQueryParams> for SubstringBucketPaginatorBuilder<C>
where
    C: ColumnTrait,
{
    fn from(value: PaginationUrlQueryParams) -> Self {
        SubstringBucketPaginatorBuilder::default()
            .offset(value.page.unwrap_or(1) - 1)
            .to_owned()
    }
}
