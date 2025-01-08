use futures_util::TryFutureExt;
use pagination::{config::SizeConfig, paginator::Paginator};
use sea_orm::{
    prelude::Expr, sea_query::SimpleExpr, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Select,
};

use crate::{
    error::{Error, Result},
    pagination::RecordsQuery,
    query::{function_name::FunctionName, select_alias::SelectAlias},
};

#[derive(serde::Serialize)]
pub struct BucketPaginator<E, C>
where
    E: EntityTrait<Column = C>,
{
    config: SizeConfig<u64>,
    buckets: Vec<String>,

    #[serde(skip_serializing)]
    column: C,

    #[serde(skip_serializing)]
    query: Select<E>,
}

impl<E, C> BucketPaginator<E, C>
where
    E: EntityTrait<Column = C>,
{
    pub fn from_query(
        conn: &DatabaseConnection,
        query: Select<E>,
        column: C,
        len: u64,
    ) -> impl std::future::Future<Output = Result<Self>> + use<'_, E, C>
    where
        C: ColumnTrait,
    {
        let select_alias = SelectAlias("__bucket_substring__");

        query
            .clone()
            .select_only()
            .column_as(
                FunctionName("substring").into_func_with_args([
                    SimpleExpr::from(column.into_expr()),
                    1.into(),
                    len.into(),
                ]),
                select_alias,
            )
            .group_by(Expr::col(select_alias))
            .into_tuple()
            .all(conn)
            .map_ok(move |value| Self {
                config: SizeConfig::default().with_last(value.len() as u64),
                buckets: value,
                column,
                query,
            })
            .map_err(Error::from)
    }
}

impl<E, C> RecordsQuery<E> for BucketPaginator<E, C>
where
    E: EntityTrait<Column = C>,
    C: ColumnTrait,
    <E as EntityTrait>::Model: Sync,
{
    fn records_query(&self, page: u64) -> sea_orm::Select<E> {
        self.query
            .clone()
            .filter(self.column.starts_with(&self.buckets[(page - 1) as usize]))
            .order_by_asc(self.column)
    }
}

impl<E, C> Paginator for BucketPaginator<E, C>
where
    E: EntityTrait<Column = C>,
{
    type Index = u64;
    type Config = SizeConfig<Self::Index>;

    fn config(&self) -> &Self::Config {
        &self.config
    }
}
