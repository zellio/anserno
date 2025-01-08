use crate::{config::Config, display::Display, index::Idx, page::Page};

/// Pagination display trait.
pub trait Paginator {
    type Index: Idx;
    type Config: Config<Index = Self::Index>;

    /// Get the configuration of the paginator.
    fn config(&self) -> &Self::Config;

    /// Create a pagination page for a given index.
    fn page(&self, page: Self::Index) -> Page<Self::Index> {
        Page {
            previous: (page > Self::Index::unsigned(1)).then(|| page - Self::Index::unsigned(1)),
            page,
            next: (page < self.config().last()).then(|| page + Self::Index::unsigned(1)),
        }
    }

    /// Generate pagination display series.
    fn series(&self, page: Self::Index) -> Vec<Display<Self::Index>> {
        let size = self.config().size();
        let last = self.config().last();

        let mut series = Vec::with_capacity(self.config().size().index());

        if size >= last {
            series.extend((Self::Index::unsigned(1).range_inclusive(last)).map(Display::Page));
        } else {
            let left = (size - Self::Index::unsigned(1)) / Self::Index::unsigned(2);

            let start = if page <= left {
                Self::Index::unsigned(1)
            } else if page > last + left - size {
                last - size + Self::Index::unsigned(1)
            } else {
                page - left
            };

            series.extend((start.range(start + size)).map(Display::Page));

            if self.config().ends() {
                series[0] = Display::Sentinel(Self::Index::unsigned(1));

                if let Some(entry) = series.get_mut(1) {
                    if entry.inner() != Self::Index::unsigned(2) {
                        *entry = Display::Gap(entry.inner());
                    }
                }

                if let Some(entry) = series.get_mut((size - Self::Index::unsigned(2)).index()) {
                    if entry.inner() != last - Self::Index::unsigned(1) {
                        *entry = Display::Gap(entry.inner());
                    }
                }

                series[(size - Self::Index::unsigned(1)).index()] = Display::Sentinel(last);
            }
        }

        if let Some(idx) = series.iter().position(|value| value.inner() == page) {
            series[idx] = Display::Selected(page);
        }

        series
    }
}
