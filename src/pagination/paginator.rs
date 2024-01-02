use sea_orm::{EntityTrait, Select};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct PaginatorDisplayConfig {
    pub start: usize,
    pub left: usize,
    pub right: usize,
    pub end: usize,
}

impl Default for PaginatorDisplayConfig {
    fn default() -> Self {
        PaginatorDisplayConfig {
            start: 1usize,
            left: 3usize,
            right: 3usize,
            end: 1usize,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum PaginatorDisplay {
    Page(usize),
    Gap(usize),
    Sentinel(usize),
    Selected(usize),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationUrlQueryParams {
    pub page: Option<usize>,
    pub items: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiPaginationUrlQueryParams {
    pub page: Option<usize>,
    pub items: Option<usize>,
}

pub trait Paginator {
    fn display_config(&self) -> PaginatorDisplayConfig;

    fn page(&self) -> usize;

    fn pages(&self) -> usize;

    fn selector<E: EntityTrait>(&self, query: Select<E>) -> Select<E>;

    fn series(&self) -> Vec<PaginatorDisplay> {
        let left_gap_start = self.display_config().start as i64 + 1;
        let mut left_gap_end = self.page() as i64 - self.display_config().left as i64 - 1;
        let mut right_gap_start = self.page() as i64 + self.display_config().right as i64 + 1;
        let right_gap_end = self.pages() as i64 - self.display_config().end as i64;

        if left_gap_end > right_gap_end {
            left_gap_end = right_gap_end
        }

        if left_gap_start > right_gap_start {
            right_gap_start = left_gap_start
        }

        let mut series = Vec::with_capacity(
            self.display_config().start
                + 1
                + self.display_config().left
                + 1
                + self.display_config().right
                + 1
                + self.display_config().end,
        );

        let mut start = 1usize;

        if left_gap_end - left_gap_start > 0 {
            series.extend((start..left_gap_start as usize).map(PaginatorDisplay::Page));
            series.push(PaginatorDisplay::Gap(
                (left_gap_end - left_gap_start + 1) as usize,
            ));
            start = (left_gap_end + 1) as usize;
        }

        if right_gap_end - right_gap_start > 0 {
            series.extend((start..right_gap_start as usize).map(PaginatorDisplay::Page));
            series.push(PaginatorDisplay::Gap(
                (right_gap_end - right_gap_start + 1) as usize,
            ));
            start = (right_gap_end + 1) as usize;
        }

        series.extend((start..=self.pages()).map(PaginatorDisplay::Page));

        if let Some(position) = series
            .iter()
            .position(|val| val == &PaginatorDisplay::Page(self.page()))
        {
            series[position] = PaginatorDisplay::Selected(self.page())
        }

        if let Some(entry @ PaginatorDisplay::Page(_)) = series.first_mut() {
            *entry = PaginatorDisplay::Sentinel(1);
        }

        if let Some(entry @ PaginatorDisplay::Page(_)) = series.last_mut() {
            *entry = PaginatorDisplay::Sentinel(self.pages());
        }

        series
    }
}
