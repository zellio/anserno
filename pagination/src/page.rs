/// A pagination page. Abstracts the logic for determining next, and previous
/// index values.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Page<I> {
    pub(crate) previous: Option<I>,
    pub(crate) page: I,
    pub(crate) next: Option<I>,
}

impl<I> Page<I> {
    /// The previous pagination index.
    #[inline]
    pub fn previous(&self) -> Option<&I> {
        self.previous.as_ref()
    }

    /// The current pagination index.
    #[inline]
    pub fn page(&self) -> &I {
        &self.page
    }

    /// The next pagination index.
    #[inline]
    pub fn next(&self) -> Option<&I> {
        self.next.as_ref()
    }
}
