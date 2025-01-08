/// Descriptive type for rendering a Paginator
#[derive(
    ::core::marker::Copy,
    ::std::clone::Clone,
    ::std::fmt::Debug,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Display<I> {
    /// A displayed number
    Page(I),
    /// A gap between sequence number
    Gap(I),
    /// I don't remember what this is for
    Sentinel(I),
    /// Currently selected page in pagination
    Selected(I),
}

impl<I> Display<I> {
    #[inline]
    pub(crate) fn inner(&self) -> I
    where
        I: ::core::marker::Copy,
    {
        match self {
            Self::Page(value)
            | Self::Gap(value)
            | Self::Sentinel(value)
            | Self::Selected(value) => *value,
        }
    }
}

impl<I> ::std::cmp::Ord for Display<I>
where
    I: std::cmp::Ord + ::core::marker::Copy,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner().cmp(&other.inner())
    }
}

impl<I> ::std::cmp::PartialOrd for Display<I>
where
    I: std::cmp::PartialOrd + ::core::marker::Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner().partial_cmp(&other.inner())
    }
}
