use derive_builder::Builder;
use sea_orm::{
    sea_query::{Expr, Func, SimpleExpr},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Select,
    SelectGetableTuple, Selector, TryGetableMany,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::{AnsernoError, AnsernoResult},
    pagination::{PaginationUrlQueryParams, Paginator, PaginatorDisplayConfig},
    queries::{FunctionName, SelectAlias},
};

pub fn substring_buckets<E, C, T>(
    bucket_column: C,
    substring_length: usize,
) -> Selector<SelectGetableTuple<T>>
where
    E: EntityTrait,
    C: ColumnTrait,
    T: TryGetableMany,
{
    let bucket_select_alias = SelectAlias("__bucket_select_alias");
    E::find()
        .select_only()
        .column_as(
            SimpleExpr::FunctionCall(Func::cust(FunctionName("substring")).args([
                bucket_column.into_expr().into(),
                1.into(),
                (substring_length as u64).into(),
            ])),
            bucket_select_alias,
        )
        .group_by(Expr::col(bucket_select_alias))
        .order_by_asc(Expr::col(bucket_select_alias))
        .into_tuple::<T>()
}

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
        query
        .order_by_asc(self.bucket_column)
        .filter(
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
