use crate::fmt;
use crate::ptr;
use crate::MaybeUninit;
use crate::{Fuse, FusedIterator};
use crate::PhantomData;
use crate::Range;

/// An iterator that copies the array elements of the base iterator.
#[derive(Debug, Clone)]
pub struct ArrayCopied<I, const N: usize> {
    iter: I,
}

impl<I, const N: usize> ArrayCopied<I, N> {
    pub(super) fn new(iter: I) -> ArrayCopied<I, N> {
        ArrayCopied { iter }
    }
}

impl<'a, I, T: 'a, const N: usize> Iterator for ArrayCopied<I, N>
where
    I: Iterator<Item = [&'a T; N]>,
    T: Copy,
{
    type Item = [T; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.map(|elem| *elem))
    }
}

impl<'a, I, T: 'a, const N: usize> ExactSizeIterator for ArrayCopied<I, N>
where
    I: ExactSizeIterator<Item = [&'a T; N]>,
    T: Copy,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, I, T: 'a, const N: usize> FusedIterator for ArrayCopied<I, N>
where
    I: FusedIterator<Item = [&'a T; N]>,
    T: Copy,
{
}

/// An iterator that clones the array elements of the base iterator.
#[derive(Debug, Clone)]
pub struct ArrayCloned<I, const N: usize> {
    iter: I,
}

impl<I, const N: usize> ArrayCloned<I, N> {
    pub(super) fn new(iter: I) -> ArrayCloned<I, N> {
        ArrayCloned { iter }
    }
}

impl<'a, I, T: 'a, const N: usize> Iterator for ArrayCloned<I, N>
where
    I: Iterator<Item = [&'a T; N]>,
    T: Clone,
{
    type Item = [T; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.map(|elem| elem.clone()))
    }
}

impl<'a, I, T: 'a, const N: usize> ExactSizeIterator for ArrayCloned<I, N>
where
    I: ExactSizeIterator<Item = [&'a T; N]>,
    T: Clone,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, I, T: 'a, const N: usize> FusedIterator for ArrayCloned<I, N>
where
    I: FusedIterator<Item = [&'a T; N]>,
    T: Clone,
{
}

/// Returns an iterator of arrays with N elements.
#[derive(Debug, Clone)]
pub struct ArrChunks<I, const N: usize> {
    pub(crate) iter: I,
}

impl<I, const N: usize> Iterator for ArrChunks<I, N>
where
    I: Iterator,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = unsafe { MaybeUninit::<[MaybeUninit<I::Item>; N]>::uninit().assume_init() };
        let mut idx = 0;

        loop {
            if let Some(elem) = self.iter.next() {
                if idx == N - 1 {
                    arr[idx] = MaybeUninit::new(elem);
                    break Some(unsafe { arr.map(|e| e.assume_init()) });
                } else {
                    arr[idx] = MaybeUninit::new(elem);
                    idx += 1;
                    continue;
                }
            } else {
                return None;
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();

        (lower / N, upper.map(|n| n / N))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count() / N
    }
}

/// Combine two iterators in pieces, specifying the length of each separately.
#[derive(Debug, Clone)]
pub struct CombineIters<I, J> {
    pub(crate) self_iter: I,
    pub(crate) self_part_len: usize,
    pub(crate) self_counter: usize,
    pub(crate) other_iter: J,
    pub(crate) other_part_len: usize,
    pub(crate) other_counter: usize,
}

impl<I, J> Iterator for CombineIters<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.self_counter == self.self_part_len {
            let Some(other_elem) = self.other_iter.next() else {
                return None;
            };
            if self.other_counter == self.other_part_len {
                self.self_counter = 0;
                self.other_counter = 0;
            }
            self.other_counter += 1;
            Some(other_elem)
        } else {
            let Some(self_elem) = self.self_iter.next() else {
                return None;
            };
            self.self_counter += 1;
            Some(self_elem)
        }
    }
}

/// An iterator that yields three elements each iteration.
#[derive(Clone)]
pub struct MapByThree<I, F> {
    iter: I,
    f: F,
}

impl<I, F> fmt::Debug for MapByThree<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("MapByThree")
            .field("iter", &self.iter)
            .field("f", &format!("{:p}", &self.f))
            //.field("f", &"Closure")
            .finish()
    }
}

impl<I, F> MapByThree<I, F> {
    pub(super) fn new(iter: I, f: F) -> MapByThree<I, F> {
        MapByThree { iter, f }
    }
}

impl<B, F, I> Iterator for MapByThree<I, F>
where
    I: Iterator,
    F: FnMut(I::Item, I::Item, I::Item) -> B,
{
    type Item = B;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(a) => match self.iter.next() {
                None => None,
                Some(b) => match self.iter.next() {
                    None => None,
                    Some(c) => Some((self.f)(a, b, c)),
                },
            },
        }
    }
}

/// An iterator that yields two elements each iteration.
#[derive(Clone)]
pub struct MapByTwo<I, F> {
    iter: I,
    f: F,
}

impl<I, F> fmt::Debug for MapByTwo<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("MapByTwo")
            .field("iter", &self.iter)
            .field("f", &format!("{:p}", &self.f))
            //.field("f", &"Closure")
            .finish()
    }
}

impl<I, F> MapByTwo<I, F> {
    pub(super) fn new(iter: I, f: F) -> MapByTwo<I, F> {
        MapByTwo { iter, f }
    }
}

impl<B, F, I> Iterator for MapByTwo<I, F>
where
    I: Iterator,
    F: FnMut(I::Item, I::Item) -> B,
{
    type Item = B;
    #[inline]
    fn next(&mut self) -> Option<B> {
        match self.iter.next() {
            None => None,
            Some(x) => match self.iter.next() {
                None => None,
                Some(y) => Some((self.f)(x, y)),
            },
        }
    }
}

/// Iterator adapter which provides two iterators for its closure at each iteration.
#[derive(Clone)]
pub struct MapIters<I, K, F> {
    iter_self: I,
    iter_other: K,
    f: F,
}

impl<I, K, F> fmt::Debug for MapIters<I, K, F>
where
    I: fmt::Debug,
    K: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("MapIters")
            .field("iter_self", &self.iter_self)
            .field("iter_other", &self.iter_other)
            .field("f", &format!("{:p}", &self.f))
            //.field("f", &"Closure")
            .finish()
    }
}

impl<I, K, F, B> MapIters<I, K, F>
where
    I: Iterator,
    K: Iterator,
    F: FnMut(&mut I, &mut K) -> Option<B>,
{
    #[inline]
    pub(super) fn new(iter_self: I, iter_other: K, f: F) -> MapIters<I, K, F> {
        MapIters {
            iter_self,
            iter_other,
            f,
        }
    }
}

impl<I, K, F, B> Iterator for MapIters<I, K, F>
where
    I: Iterator,
    K: Iterator,
    F: FnMut(&mut I, &mut K) -> Option<B>,
{
    type Item = B;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.iter_self, &mut self.iter_other)
    }
}

/// An iterator adapter that preserves the element of the last iteration.
#[derive(Debug, Clone)]
pub struct LastTaken<I: Iterator> {
    iter: I,
    item: Option<I::Item>,
}

impl<I: Iterator> LastTaken<I> {
    pub(super) fn new(iter: I, item: Option<I::Item>) -> LastTaken<I> {
        LastTaken { iter, item }
    }
}

impl<I: Iterator> LastTaken<I> {
    /// Returns the iterator element of the last iteration.
    ///
    /// Using the LastTaken iterator adapter, you can get the value of
    /// the last iteration of the iterator without moving the iterator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222];
    /// let mut iter = arr.into_iter().last_taken();
    /// let vec = iter.by_ref().map_while(|e| { e.checked_add(200) }).collect::<Vec<_>>();
    /// assert_eq!(Some(66), iter.last_item().copied());
    /// ```
    #[inline]
    pub fn last_item(&self) -> Option<&I::Item> {
        self.item.as_ref()
    }
}

impl<I> Iterator for LastTaken<I>
where
    I: Iterator,
    <I as Iterator>::Item: Copy,
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.item = Some(self.iter.next()?);
        self.item
    }
}

impl<I> ExactSizeIterator for LastTaken<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: Copy,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// An iterator adapter that retains the current element on each iteration.
#[derive(Debug, Clone)]
pub struct Previous<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
    item: I::Item,
}

impl<I> Previous<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub(super) fn new(iter: I, item: I::Item) -> Previous<I> {
        Previous { iter, item }
    }
}

impl<I> Iterator for Previous<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.item.clone();
        self.item = self.iter.next()?;
        Some((prev, self.item.clone()))
    }
}

impl<I> ExactSizeIterator for Previous<I>
where
    I: ExactSizeIterator,
    <I as Iterator>::Item: Copy,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// Iterator adapter with skip and step capabilities in one.
#[derive(Debug, Clone)]
pub struct SkipStepBy<I> {
    pub(crate) iter: Fuse<I>,
    pub(crate) skip: usize,
    pub(crate) step: usize,
}

impl<I> Iterator for SkipStepBy<I>
where
    I: Iterator,
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let elem = self.iter.nth(self.skip);
        self.skip = self.step;
        elem
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let div = |x: usize| {
            if x == 0 {
                0
            } else {
                (1 + (x - 1) / (self.step + 1)).saturating_sub(self.skip)
            }
        };

        (div(lower), upper.map(div))
    }
}

impl<I> ExactSizeIterator for SkipStepBy<I> where I: ExactSizeIterator {}

/// Iterator adapter, with variable step.
#[derive(Clone)]
pub struct StepByFn<I, F> {
    pub(crate) iter: Fuse<I>,
    pub(crate) skip: usize,
    pub(crate) f: F,
}

impl<I, F> fmt::Debug for StepByFn<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("StepByFn")
            .field("iter", &self.iter)
            .field("skip", &self.skip)
            .field("f", &format!("{:p}", &self.f))
            .finish()
    }
}

impl<I, F> Iterator for StepByFn<I, F>
where
    I: Iterator,
    F: FnMut(&mut usize) -> usize,
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ret = (self.f)(&mut self.skip);
        if ret == 0 {
            return None;
        }
        self.iter.nth(ret - 1)
    }
}

/// An iterator that copies the slice elements of the base iterator at each iteration.
#[derive(Debug, Clone)]
pub struct SliceCopied<I, const N: usize> {
    iter: I,
}

impl<I, const N: usize> SliceCopied<I, N> {
    pub(super) fn new(iter: I) -> SliceCopied<I, N> {
        SliceCopied { iter }
    }
}

impl<'a, I, T: 'a, const N: usize> Iterator for SliceCopied<I, N>
where
    I: Iterator<Item = &'a [T]>,
    T: Copy,
{
    type Item = [T; N];
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(copy_from_slice::<T, N>(self.iter.next()?))
    }
}

impl<'a, I, T: 'a, const N: usize> ExactSizeIterator for SliceCopied<I, N>
where
    I: Iterator<Item = &'a [T]>,
    T: Copy,
{
}

#[inline]
fn copy_from_slice<T, const N: usize>(slice: &[T]) -> [T; N]
where
    T: Copy,
{
    assert!(slice.len() == N, "slice must be equal to array");
    let mut arr = MaybeUninit::<[T; N]>::uninit();
    unsafe {
        ptr::copy(slice.as_ptr(), arr.as_mut_ptr() as *mut T, N);
        arr.assume_init()
    }
}

/// Immutable tuple iterator.
#[derive(Debug, Clone)]
pub struct TupleImut<'a, T, const N: usize> {
    pub(crate) arr: [&'a T; N],
    pub(crate) idx_iter: Range<usize>,
}
/// Mutable tuple iterator.
#[derive(Debug, Clone)]
pub struct TupleMut<'a, T: 'a, const N: usize> {
    pub(crate) arr: [*mut T; N],
    pub(crate) idx_iter: Range<usize>,
    pub(crate) _unused: PhantomData<&'a mut T>,
}
