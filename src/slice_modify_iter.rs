use crate::{SliceIndex, IterExtd, StepBoundary};
use crate::{Range, RangeInclusive};
use crate::PhantomData;

impl<T> SliceModifyIter<T> for [T] {}

/// Iterator with external slice indexing.
pub trait SliceModifyIter<T>
where
    Self: AsMut<[T]> + AsRef<[T]>,
{
    /// Create an iterator that returns [`RangeInclusive`] structures with index bounds from the slice.
    ///
    /// # Panics
    ///
    /// Panics if the step size is zero.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::SliceModifyIter;
    ///
    /// let val = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    ///
    /// let iter = val.gen_range_bounds(3);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![0..=2, 3..=5, 6..=8, 9..=10]);
    /// ```
    fn gen_range_bounds(&self, size: usize) -> GenRangeBounds {
        assert!(size != 0);
        GenRangeBounds {
            iter: (0..self.as_ref().len()).step_boundary(size),
        }
    }

    /// Create an iterator that returns tuples with index bounds from the slice.
    ///
    /// # Panics
    ///
    /// Panics if the step size is zero.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::SliceModifyIter;
    ///
    /// let val = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let iter = val.gen_tuple_bounds(3);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(0, 2), (3, 5), (6, 8), (9, 10)]);
    /// ```
    fn gen_tuple_bounds(&self, size: usize) -> StepBoundary<Range<usize>> {
        assert!(size != 0);
        (0..self.as_ref().len()).step_boundary(size)
    }

    /// Modify a slice using an iterator with external slice indexing.
    ///
    /// ```
    /// use iterextd::SliceModifyIter;
    ///
    /// let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let iter = vec.gen_range_bounds(2);
    /// let logic = |e: &mut[i32]| {
    ///     if e.len() == 2 {
    ///         let one = e[0];
    ///         e[0] = e[1];
    ///         e[1] = one;
    ///     }
    /// };
    /// let _ = vec.modify_slice(iter, logic);
    /// assert_eq!(vec, vec![1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 10]);
    /// ```
    fn modify_slice<F, I>(&mut self, iter: I, f: F)
    where
        I: Iterator,
        <I as Iterator>::Item: SliceIndex<[T], Output = [T]>,
        F: FnMut(&mut [T]),
    {
        SliceModify::<T, I>::new(self.as_mut(), iter).for_each(f);
    }
}

/// An iterator that allows creating RangeInclusive slice boundaries.
#[derive(Debug, Clone)]
pub struct GenRangeBounds {
    iter: StepBoundary<Range<usize>>,
}

impl Iterator for GenRangeBounds {
    type Item = RangeInclusive<usize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let tuple = self.iter.next()?;
        Some(RangeInclusive::new(tuple.0, tuple.1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl ExactSizeIterator for GenRangeBounds {}

/// An iterator that allows modification of a slice.
#[derive(Debug, Clone)]
pub struct SliceModify<'a, T: 'a, I>
where
    I: Iterator,
    <I as Iterator>::Item: SliceIndex<[T], Output = [T]>,
{
    ptr: *mut [T],
    iter: I,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: 'a, I> SliceModify<'a, T, I>
where
    I: Iterator,
    <I as Iterator>::Item: SliceIndex<[T], Output = [T]>,
{
    fn new(slice: &'a mut [T], iter: I) -> Self {
        Self {
            ptr: slice,
            iter,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, I> Iterator for SliceModify<'a, T, I>
where
    I: Iterator,
    <I as Iterator>::Item: SliceIndex<[T], Output = [T]>,
{
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.iter.next()?;
        Some(unsafe { &mut (*self.ptr)[idx] })
    }
}
