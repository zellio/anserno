use futures_util::TryFutureExt;
use pagination::{config::SizeConfig, paginator::Paginator};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect};

use crate::{
    error::{Error, Result},
    pagination::RecordsQuery,
};

#[derive(serde::Serialize)]
pub struct QueryPaginator<E>
where
    E: EntityTrait,
{
    #[serde(skip_serializing)]
    query: sea_orm::Select<E>,

    page_length: u64,
    count: u64,
    config: SizeConfig<u64>,
}

impl<E> QueryPaginator<E>
where
    E: EntityTrait,
{
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn with_page_length(self, page_length: u64) -> Self {
        let mut query_pagiantor = self;

        query_pagiantor.page_length = page_length;

        query_pagiantor.config = query_pagiantor
            .config
            .with_last(query_pagiantor.count.div_ceil(page_length));

        query_pagiantor
    }

    pub fn from_query(
        conn: &DatabaseConnection,
        query: sea_orm::Select<E>,
    ) -> impl ::std::future::Future<Output = Result<Self>> + use<'_, E>
    where
        <E as EntityTrait>::Model: ::core::marker::Sync,
    {
        query
            .clone()
            .count(conn)
            .map_ok(move |count| Self {
                query,
                page_length: 12,
                count,
                config: SizeConfig::default().with_last(count.div_ceil(12)),
            })
            .map_err(Error::from)
    }
}

impl<E> RecordsQuery<E> for QueryPaginator<E>
where
    E: EntityTrait,
    <E as EntityTrait>::Model: Sync,
{
    fn records_query(&self, page: u64) -> sea_orm::Select<E> {
        self.query
            .clone()
            .offset(self.page_length * (page - 1))
            .limit(self.page_length)
    }
}

impl<E> Paginator for QueryPaginator<E>
where
    E: EntityTrait,
{
    type Index = u64;
    type Config = SizeConfig<Self::Index>;

    fn config(&self) -> &Self::Config {
        &self.config
    }
}
