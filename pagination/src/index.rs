pub trait Idx:
    ::std::marker::Copy
    + ::std::ops::Add<Output = Self>
    + ::std::ops::Sub<Output = Self>
    + ::std::ops::Mul<Output = Self>
    + ::std::ops::Div<Output = Self>
    + ::std::cmp::PartialOrd
{
    fn unsigned(idx: usize) -> Self;

    fn signed(idx: isize) -> Self;

    fn zero() -> Self;

    fn index(self) -> usize;

    type RangeIter: Iterator<Item = Self> + ::std::ops::RangeBounds<Self>;

    fn range(self, end: Self) -> Self::RangeIter;

    type RangeInclusiveIter: Iterator<Item = Self> + ::std::ops::RangeBounds<Self>;

    fn range_inclusive(self, end: Self) -> Self::RangeInclusiveIter;
}

macro_rules! impl_idx {
    ($TYPE:ty) => {
        impl Idx for $TYPE {
            #[inline(always)]
            fn unsigned(idx: usize) -> Self {
                assert!(idx <= <$TYPE>::MAX as usize);
                idx as $TYPE
            }

            #[inline(always)]
            fn signed(idx: isize) -> Self {
                assert!(<$TYPE>::MIN as isize <= idx && idx <= <$TYPE>::MAX as isize);
                idx as $TYPE
            }

            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn index(self) -> usize {
                self as usize
            }

            type RangeIter = ::std::ops::Range<Self>;

            #[inline(always)]
            fn range(self, end: Self) -> Self::RangeIter {
                self..end
            }

            type RangeInclusiveIter = ::std::ops::RangeInclusive<Self>;

            #[inline(always)]
            fn range_inclusive(self, end: Self) -> Self::RangeInclusiveIter {
                self..=end
            }
        }
    };
}

impl_idx!(u8);
impl_idx!(u16);
impl_idx!(u32);
impl_idx!(u64);
impl_idx!(usize);

impl_idx!(i8);
impl_idx!(i16);
impl_idx!(i32);
impl_idx!(i64);
impl_idx!(isize);
