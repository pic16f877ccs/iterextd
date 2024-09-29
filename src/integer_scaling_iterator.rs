use crate::FusedIterator;
use crate::Itertools;
use crate::PhantomData;
use crate::TryFromByAdd;
use crate::{one, zero, Bounded, CheckedMul, CheckedSub, One, Zero};
use crate::{Add, Div, Mul, Sub};
use crate::{MinMax, NoElements, OneElement};

/// Structure that stores data, parameters for the `scaling` iterator adapter.
#[derive(Debug, Clone)]
pub struct Scaling<I: Iterator, U, R> {
    denominator: U,
    numerator: U,
    input_start: U,
    output_start: U,
    iter: I,
    phantom: PhantomData<R>,
}

impl<I, U, R> Scaling<I, U, R>
where
    I: Iterator + Clone,
    I::Item: PartialOrd + Copy,
    U: TryFromByAdd<R>
        + TryFromByAdd<I::Item>
        + convert_by_add::FromByAddAll<I::Item>
        + CheckedMul
        + CheckedSub
        + PartialEq
        + One
        + Zero
        + Copy,
    R: Bounded + Copy,
{
    #[inline]
    pub(crate) fn new(iter: I, output_range: impl convert_by_add::RangeBoundsInner<R>) -> Self {
        let output_start = U::try_from_by_add(match output_range.start_bound() {
            convert_by_add::BoundInner::Included(&start) => start,
            convert_by_add::BoundInner::Unbounded => <R>::min_value(),
        })
        .expect("overflow of the selected intermediate type");

        let output_end = U::try_from_by_add(match output_range.end_bound() {
            convert_by_add::BoundInner::Included(&end) => end,
            convert_by_add::BoundInner::Unbounded => <R>::max_value(),
        })
        .expect("overflow of the selected intermediate type");

        let (min, max) = match iter.clone().minmax() {
            NoElements => (zero::<U>(), zero::<U>()),
            OneElement(max) => {
                let max_converted =
                    U::try_from_by_add(max).expect("overflow of the selected intermediate type");
                (zero::<U>(), max_converted)
            }
            MinMax(min, max) => {
                let max_converted =
                    U::try_from_by_add(max).expect("overflow of the selected intermediate type");
                if min == max {
                    (zero::<U>(), max_converted)
                } else {
                    (U::from_by_add_all(min), max_converted)
                }
            }
        };

        let _ = max
            .checked_mul(&(output_end))
            .expect("overflow of the selected intermediate type");
        Self {
            iter,
            denominator: if max == zero::<U>() {
                one::<U>()
            } else {
                max - min
            },
            numerator: output_end
                .checked_sub(&output_start)
                .expect("lower range bound must be less than upper"),
            input_start: min,
            output_start,
            phantom: PhantomData,
        }
    }
}

impl<I, U, R> Iterator for Scaling<I, U, R>
where
    I: Iterator,
    U: convert_by_add::FromByAddAll<I::Item>
        + Sub<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Add<Output = U>
        + Copy,
    R: convert_by_add::FromByAddAll<U>,
{
    type Item = R;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|val| {
            R::from_by_add_all(
                (U::from_by_add_all(val) - self.input_start) * self.numerator / self.denominator
                    + self.output_start,
            )
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T: ?Sized, R> Scaler<R> for T
where
    T: Iterator,
    R: Bounded + Copy,
{
}

/// An `Iterator` that scales the values of the input iterator to the specified range.
pub trait Scaler<R>: Iterator
where
    R: Bounded + Copy,
{
    #[inline]
    /// Scale the values of the input iterator to the specified range.
    ///
    /// # Panics
    ///
    /// If the input or output value cannot be converted to the selected intermediate type.
    /// Panic when the lower bound of the range is greater than the upper bound.
    /// Overflow when multiplying maximum input and output values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lesson_bithacks::Scaler;
    ///
    /// let iter = (0..=5u8).scaling::<u16>(..70u8);
    /// assert_eq!(iter.collect::<Vec<_>>(), vec![0, 14, 28, 42, 56, 70]);
    /// ```
    fn scaling<U>(
        self,
        output_range: impl convert_by_add::RangeBoundsInner<R>,
    ) -> Scaling<Self, U, R>
    where
        Self::Item: PartialOrd + Copy,
        U: Sub<Output = U>
            + TryFromByAdd<R>
            + TryFromByAdd<Self::Item>
            + convert_by_add::FromByAddAll<Self::Item>
            + PartialEq
            + CheckedSub
            + CheckedMul
            + One
            + Zero
            + Copy,
        Self: Sized + Clone,
    {
        Scaling::new(self, output_range)
    }
}

impl<I, U, R> ExactSizeIterator for Scaling<I, U, R>
where
    I: ExactSizeIterator,
    U: convert_by_add::FromByAddAll<I::Item>
        + Sub<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Add<Output = U>
        + Copy,
    R: convert_by_add::FromByAddAll<U>,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}
impl<I, U, R> FusedIterator for Scaling<I, U, R>
where
    I: FusedIterator,
    U: convert_by_add::FromByAddAll<I::Item>
        + Sub<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Add<Output = U>
        + Copy,
    R: convert_by_add::FromByAddAll<U>,
{
}

impl<I, U, R> DoubleEndedIterator for Scaling<I, U, R>
where
    I: DoubleEndedIterator,
    U: convert_by_add::FromByAddAll<I::Item>
        + Sub<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Add<Output = U>
        + Copy,
    R: convert_by_add::FromByAddAll<U>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|val| {
            R::from_by_add_all(
                (U::from_by_add_all(val) - self.input_start) * self.numerator / self.denominator
                    + self.output_start,
            )
        })
    }
}

pub(crate) mod convert_by_add {
    use std::ops::{RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
    pub trait FromByAddAll<T> {
        fn from_by_add_all(n: T) -> Self
        where
            Self: Sized;
    }

    macro_rules! signed_or_unsigned_impls {
        ( $( $from_type:ty; $($into_type:ty),* );* ) => {
            $(
                impl FromByAddAll<$from_type> for $from_type {
                    #[inline]
                    fn from_by_add_all(n: Self) -> Self {
                        n
                    }
                }

                $(
                    impl FromByAddAll<$from_type> for $into_type {
                        #[inline]
                        fn from_by_add_all(n: $from_type) -> Self {
                            n as Self
                        }
                    }
                )*
            )*
        }
    }

    signed_or_unsigned_impls! { i8; i16, i32, i64, isize, i128; i16; i32, i64, isize, i128;
    i32; i64, isize, i128; i64; isize, i128; isize; i64, i128; i128; }
    signed_or_unsigned_impls! { u8; u16, u32, u64, usize, u128; u16; u32, u64, usize, u128;
    u32; u64, usize, u128; u64; usize, u128; usize; u64, u128; u128; }

    macro_rules! signed_gt_signed_impls {
        ( $( $into_type:ty; $($from_type:ty),* );* ) => {
            $(
                $(
                    impl FromByAddAll<$from_type> for $into_type {
                        #[inline]
                        fn from_by_add_all(n: $from_type) -> Self {
                            n as Self
                        }
                    }
                )*
            )*
        }
    }

    signed_gt_signed_impls! { i8; i16, i32, i64, isize, i128; i16; i32, i64, isize, i128;
    i32; i64, isize, i128; i64; i128; isize; i128  }

    macro_rules! signed_gt_unsigned_impls {
        ( $( $into_type:ty; $($from_type:ty),* );* ) => {
            $(
                $(
                    impl FromByAddAll<$from_type> for $into_type {
                        #[inline]
                        fn from_by_add_all(n: $from_type) -> Self {
                            ((n as Self).wrapping_add(Self::MAX)).wrapping_add(1)
                        }
                    }
                )*
            )*
        }
    }

    signed_gt_unsigned_impls! { i8; u16, u32, u64, usize, u128; i16; u32, u64, usize, u128;
    i32; u64, usize, u128; i64; u128; isize; u128 }

    macro_rules! unsigned_gt_unsigned_impls {
        ( $( $into_type:ty; $($from_type:ty),* );* ) => {
            $(
                $(
                    impl FromByAddAll<$from_type> for $into_type {
                        #[inline]
                        fn from_by_add_all(n: $from_type) -> Self {
                            n as Self
                        }
                    }
                )*
            )*
        }
    }

    unsigned_gt_unsigned_impls! { u8; u16, u32, u64, usize, u128; u16; u32, u64, usize, u128;
    u32; u64, usize, u128; u64; u128; usize; u128 }

    macro_rules! unsigned_gt_signed_impls {
        ( $into_type:ty, $add_value:expr; $($from_type:ty),* ) => {
            $(
                impl FromByAddAll<$from_type> for $into_type {
                    #[inline]
                    fn from_by_add_all(n: $from_type) -> Self {
                        (n as Self).wrapping_add($add_value)
                    }
                }
            )*
        }
    }

    unsigned_gt_signed_impls! { u8, 128; i16, i32, i64, isize, i128 }
    unsigned_gt_signed_impls! { u16, 32_768; i32, i64, isize, i128 }
    unsigned_gt_signed_impls! { u32, 2_147_483_648; i64, isize, i128 }
    unsigned_gt_signed_impls! { u64, 9_223_372_036_854_775_808; i128 }
    unsigned_gt_signed_impls! { usize, 9_223_372_036_854_775_808; i128 }

    macro_rules! unsigned_to_signed_impls {
        ( $from_type:ty, $as_type:ty; $($into_type:ty),* ) => {
            impl FromByAddAll<$from_type> for $as_type {
                #[inline]
                fn from_by_add_all(n: $from_type) -> Self {
                    ((n as Self).wrapping_add(<Self>::MAX)).wrapping_add(1)
                }
            }

            $(
                impl FromByAddAll<$from_type> for $into_type {
                    #[inline]
                    fn from_by_add_all(n: $from_type) -> Self {
                        ((n as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1) as Self
                    }
                }
            )*
        };
    }

    unsigned_to_signed_impls! { u8, i8; i16, i32, i64, isize, i128 }
    unsigned_to_signed_impls! { u16, i16; i32, i64, isize, i128 }
    unsigned_to_signed_impls! { u32, i32; i64, isize, i128 }
    unsigned_to_signed_impls! { u64, i64; isize, i128 }
    unsigned_to_signed_impls! { usize, isize; i64, i128 }
    unsigned_to_signed_impls! { u128, i128; }

    macro_rules! signed_to_unsigned_impls {
        ( $from_type:ty, $add_value:expr; $($into_type:ty),*) => {
            $(
                impl FromByAddAll<$from_type> for $into_type {
                    #[inline]
                    fn from_by_add_all(n: $from_type) -> Self {
                        (n as Self).wrapping_add($add_value)
                    }
                }
            )*
        };
    }

    signed_to_unsigned_impls! { i8, 128; u8, u16, u32, u64, usize, u128 }
    signed_to_unsigned_impls! { i16, 32_768; u16, u32, u64, usize, u128 }
    signed_to_unsigned_impls! { i32, 2_147_483_648; u32, u64, usize, u128 }
    signed_to_unsigned_impls! { i64, 9_223_372_036_854_775_808; u64, usize, u128 }
    signed_to_unsigned_impls! { isize, 9_223_372_036_854_775_808; u64, usize, u128 }
    signed_to_unsigned_impls! { i128, (i128::MAX as u128) + 1; u128 }

    #[derive(Debug, Clone)]
    pub enum BoundInner<T> {
        Included(T),
        Unbounded,
    }

    pub trait RangeBoundsInner<T>
    where
        T: ?Sized,
    {
        fn start_bound(&self) -> BoundInner<&T>;
        fn end_bound(&self) -> BoundInner<&T>;
    }

    impl<T: ?Sized> RangeBoundsInner<T> for RangeFull {
        fn start_bound(&self) -> BoundInner<&T> {
            BoundInner::Unbounded
        }
        fn end_bound(&self) -> BoundInner<&T> {
            BoundInner::Unbounded
        }
    }

    impl<T> RangeBoundsInner<T> for RangeInclusive<T> {
        fn start_bound(&self) -> BoundInner<&T> {
            BoundInner::Included(self.start())
        }
        fn end_bound(&self) -> BoundInner<&T> {
            BoundInner::Included(self.end())
        }
    }

    impl<T> RangeBoundsInner<T> for RangeToInclusive<T> {
        fn start_bound(&self) -> BoundInner<&T> {
            BoundInner::Unbounded
        }
        fn end_bound(&self) -> BoundInner<&T> {
            BoundInner::Included(&self.end)
        }
    }

    impl<T> RangeBoundsInner<T> for RangeTo<T> {
        fn start_bound(&self) -> BoundInner<&T> {
            BoundInner::Unbounded
        }
        fn end_bound(&self) -> BoundInner<&T> {
            BoundInner::Included(&self.end)
        }
    }

    impl<T> RangeBoundsInner<T> for RangeFrom<T> {
        fn start_bound(&self) -> BoundInner<&T> {
            BoundInner::Included(&self.start)
        }
        fn end_bound(&self) -> BoundInner<&T> {
            BoundInner::Unbounded
        }
    }

    impl<T> RangeBoundsInner<T> for (T, T) {
        fn start_bound(&self) -> BoundInner<&T> {
            BoundInner::Included(&self.0)
        }
        fn end_bound(&self) -> BoundInner<&T> {
            BoundInner::Included(&self.1)
        }
    }
}
