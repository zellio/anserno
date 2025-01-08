use crate::index::Idx;

pub trait Config {
    type Index: Idx;

    /// Returns the count of the series being paginated.
    fn size(&self) -> Self::Index;

    /// Returns the last page number of the series being paginated.
    fn last(&self) -> Self::Index;

    /// Retrurns true if the sequence is finite.
    fn ends(&self) -> bool;
}

#[derive(::core::marker::Copy, ::std::clone::Clone, ::std::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SizeConfig<I> {
    size: I,
    last: I,
    ends: bool,
}

impl<I> SizeConfig<I> {
    /// Sets the size of a SizeConfig.
    pub fn with_size(self, size: I) -> Self {
        let mut size_config = self;
        size_config.size = size;
        size_config
    }

    /// Sets the last of a SizeConfig.
    pub fn with_last(self, last: I) -> Self {
        let mut size_config = self;
        size_config.last = last;
        size_config
    }

    /// Sets the ends flag of a SizeConfig.
    pub fn with_ends(self, ends: bool) -> Self {
        let mut size_config = self;
        size_config.ends = ends;
        size_config
    }
}

impl<I> ::std::default::Default for SizeConfig<I>
where
    I: Idx,
{
    fn default() -> Self {
        Self {
            size: I::unsigned(9),
            last: I::zero(),
            ends: true,
        }
    }
}

impl<I> Config for SizeConfig<I>
where
    I: Idx,
{
    type Index = I;

    #[inline]
    fn size(&self) -> Self::Index {
        self.size
    }

    #[inline]
    fn last(&self) -> Self::Index {
        self.last
    }

    #[inline]
    fn ends(&self) -> bool {
        self.ends
    }
}
