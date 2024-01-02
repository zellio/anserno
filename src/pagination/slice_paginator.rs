use derive_builder::Builder;
use sea_orm::{query::QuerySelect, EntityTrait, Select};
use serde::Serialize;

use crate::{
    error::AnsernoError,
    pagination::{
        ApiPaginationUrlQueryParams, PaginationUrlQueryParams, Paginator, PaginatorDisplayConfig,
    },
};

#[derive(Builder, Debug, Serialize)]
#[builder(build_fn(error = "AnsernoError"), setter(into))]
pub struct SlicePaginator {
    pub count: usize,

    #[builder(default = "0usize")]
    pub offset: usize,

    #[builder(default = "12usize")]
    pub items: usize,

    #[builder(default = "PaginatorDisplayConfig::default()")]
    pub display_config: PaginatorDisplayConfig,
}

impl Paginator for SlicePaginator {
    fn display_config(&self) -> PaginatorDisplayConfig {
        self.display_config
    }

    fn page(&self) -> usize {
        self.offset + 1
    }

    fn pages(&self) -> usize {
        std::cmp::max((self.count + self.items - 1) / self.items, 1usize)
    }

    fn selector<E: EntityTrait>(&self, query: Select<E>) -> Select<E> {
        query
            .offset((self.offset * self.items) as u64)
            .limit(self.items as u64)
    }
}

impl From<PaginationUrlQueryParams> for SlicePaginatorBuilder {
    fn from(value: PaginationUrlQueryParams) -> Self {
        SlicePaginatorBuilder::default()
            .items(value.items.unwrap_or(12usize))
            .offset(value.page.unwrap_or(1) - 1)
            .to_owned()
    }
}

impl From<ApiPaginationUrlQueryParams> for SlicePaginatorBuilder {
    fn from(value: ApiPaginationUrlQueryParams) -> Self {
        SlicePaginatorBuilder::default()
            .items(value.items.unwrap_or(128usize))
            .offset(value.page.unwrap_or(1) - 1)
            .to_owned()
    }
}
