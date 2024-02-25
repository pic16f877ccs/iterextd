use crate::structs::{
    ArrChunks, ArrayCloned, ArrayCopied, CombineIters, LastTaken, MapByThree, MapByTwo, MapIters,
    Previous, SkipStepBy, SliceCopied, StepByFn,
};
use crate::FusedIterator;
use crate::MaybeUninit;

impl<T: ?Sized> IterExtd for T where T: Iterator {}

impl<I, const N: usize> FusedIterator for ArrChunks<I, N> where I: FusedIterator {}

impl<I, const N: usize> ExactSizeIterator for ArrChunks<I, N>
where
    I: ExactSizeIterator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len() / N
    }
}

/// Trait extends the functionality of the standard iterator.
///
/// This trait provides additional methods for working with iterators, enhancing their functionality.
pub trait IterExtd: Iterator {
    /// Returns an iterator over the N elements of the base iterator per iteration.
    ///
    /// # Panics
    /// Panics if N is 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = "functions";
    /// let iter = arr.chars().arr_chunks();
    /// let vec_arrs: Vec<[char; 4]> = iter.collect();
    /// assert_eq!(vec_arrs, vec![['f', 'u', 'n', 'c'], ['t', 'i', 'o', 'n']]);
    /// ```
    fn arr_chunks<const N: usize>(self) -> ArrChunks<Self, N>
    where
        Self: Sized,
    {
        assert!(N != 0, "chunk size must be non-zero");
        ArrChunks { iter: self }
    }

    /// Creates an iterator that clones all elements of its arrays.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let vec = vec!["iter".to_string(), "array".to_string(), "chunk".to_string(), "pointer".to_string()];
    /// let iter = vec.iter().arr_chunks::<2>().array_cloned();
    /// let vec_cloned = iter.collect::<Vec<_>>();
    /// assert_eq!(vec_cloned, vec![["iter".to_string(), "array".to_string()], ["chunk".to_string(), "pointer".to_string()]]);
    /// ```
    fn array_cloned<'a, T: 'a, const N: usize>(self) -> ArrayCloned<Self, N>
    where
        Self: Sized + Iterator<Item = [&'a T; N]>,
        T: Clone,
    {
        ArrayCloned::new(self)
    }

    /// Creates an iterator that copies all elements of its arrays.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let iter = vec.iter().arr_chunks::<2>().array_copied();
    /// let vec_copied = iter.collect::<Vec<_>>();
    /// assert_eq!(vec_copied, vec![[1, 2], [3, 4]]);
    /// ```
    fn array_copied<'a, T: 'a, const N: usize>(self) -> ArrayCopied<Self, N>
    where
        Self: Sized + Iterator<Item = [&'a T; N]>,
        T: Copy,
    {
        ArrayCopied::new(self)
    }

    /// Collect a zeroed array for nullable types.
    ///
    /// # Panics
    /// Panics if N is 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let val = "The current year is 2024.";
    /// let iter = val.chars().filter(|elem| elem.is_ascii_digit());
    /// let arr: (usize, [char; 8]) = iter.collect_arr_zeroed();
    /// assert_eq!(arr, (4, ['2', '0', '2', '4', '\0', '\0', '\0', '\0']));
    /// ```
    fn collect_arr_zeroed<const N: usize>(self) -> (usize, [Self::Item; N])
    where
        Self: Sized + Iterator,
        Self::Item: AllowZero,
        //[Self::Item]: AllowZero,
    {
        assert!(N != 0, "array size must be non-zero");
        let mut index = 0;
        let mut arr: [Self::Item; N] = unsafe { MaybeUninit::zeroed().assume_init() };
        for (idx, elem) in self.enumerate() {
            if idx == N {
                break;
            }
            arr[idx] = elem;
            index = idx;
        }
        (index + 1, arr)
    }

    /// Create an array from an iterator.
    ///
    /// # Panics
    /// Panics if N is 0 or N greater than the length of the iterator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let val = vec!["The", "current", "year", "is", "2024."];
    /// let iter = val.into_iter();
    /// let arr: [&str; 2] = iter.collect_array();
    /// assert_eq!(arr, ["The", "current"]);
    /// ```
    fn collect_array<const N: usize>(self) -> [<Self as Iterator>::Item; N]
    where
        Self: Sized + Iterator,
    {
        assert!(N != 0, "chunk size must be non-zero");
        self.arr_chunks::<N>().next().unwrap()
    }

    /// Consumes an iterator, returns nothing.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let mut arr = [200, 201, 202, 203];
    /// let _ = arr.iter_mut().map(|elem| { *elem +=10;}).consume();
    /// assert_eq!(arr, [210, 211, 212, 213]);
    ///
    /// ```
    fn consume(mut self)
    where
        Self: Sized,
    {
        while let Some(_) = self.next() { }
    }

    /// Combine two iterators in parts sequentially.
    /// The length of the piece can be set for each separately.
    ///
    /// Panics if both `basic_repeats` and `other_repeats` are 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let rgb_iter = [200u8, 110, 25, 73, 49, 155, 16, 57, 180].into_iter();
    /// let alpha_iter = [10u8, 20, 30, 40, 50].into_iter();
    /// let rgba = rgb_iter.combine_iters(3, alpha_iter, 1).collect::<Vec<u8>>();
    /// assert_eq!(rgba, vec![200u8, 110, 25, 10, 73, 49, 155, 20, 16, 57, 180, 30]);
    /// ```
    fn combine_iters<J>(
        self,
        self_part_len: usize,
        other_iter: J,
        other_part_len: usize,
    ) -> CombineIters<Self, J>
    where
        J: Iterator,
        Self: Sized,
    {
        assert!(
            (self_part_len != 0) || (other_part_len != 0),
            "one of repeats  must be non-zero"
        );
        let self_part_len = if other_part_len == 0 {
            usize::MAX
        } else {
            self_part_len
        };
        CombineIters {
            self_iter: self,
            self_part_len,
            self_counter: 0,
            other_iter,
            other_part_len,
            other_counter: 1,
        }
    }

    /// Creates an iterator that yields two elements per iteration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let iter = arr.iter().map_by_two(|a, b| { Into::into(*a + *b) });
    /// let vec = iter.collect::<Vec<u32>>();
    /// assert_eq!(vec, vec![3, 7, 11, 15, 19]);
    /// ```
    fn map_by_two<B, F>(self, f: F) -> MapByTwo<Self, F>
    where
        F: FnMut(Self::Item, Self::Item) -> B,
        Self: Sized,
    {
        MapByTwo::new(self, f)
    }

    /// Creates an iterator that yields three elements per iteration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    /// let iter = arr.into_iter().map_by_three(|a, b, c| { [c, a, b] } );
    /// let vec = iter.flatten().collect::<Vec<_>>();
    /// assert_eq!(vec, vec![2, 0, 1, 5, 3, 4, 8, 6, 7, 11, 9, 10, 14, 12, 13]);
    /// ```
    fn map_by_three<B, F>(self, f: F) -> MapByThree<Self, F>
    where
        F: FnMut(Self::Item, Self::Item, Self::Item) -> B,
        Self: Sized,
    {
        MapByThree::new(self, f)
    }

    /// Creates an iterator that yields two iterators per iteration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let self_arr: [u16; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let oth_arr: [&u8; 5] = [&20, &21, &22, &23, &24];
    /// let self_iter = self_arr.iter();
    /// let other_iter = oth_arr.iter();
    /// let iter = self_iter
    ///     .map_iters(
    ///         other_iter.clone(), |s, o| { Some([*s.next()?, *s.next()?, *s.next()?, (*(*o.next()?)).into()]) }
    ///     );
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![[1, 2, 3, 20], [4, 5, 6, 21], [7, 8, 9, 22]]);
    /// ```
    fn map_iters<K, F, B>(self, k: K, f: F) -> MapIters<Self, K, F>
    where
        K: Iterator,
        F: FnMut(&mut Self, &mut K) -> Option<B>,
        Self: Sized,
    {
        MapIters::new(self, k, f)
    }

    /// The iterator adapter provides the ability to obtain a tuple of two values (last, current) at each iteration.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let val = 16;
    /// let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let iter = arr.iter().previous(&val).map(|(p, e)| { *p + *e });
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![17, 3, 5, 7, 9, 11, 13, 15, 17, 19]);
    /// ```
    fn previous(self, item: Self::Item) -> Previous<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Previous::new(self, item)
    }

    /// An iterator adapter provides the ability to retrieve the last returned value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let mut iter = arr.into_iter().last_taken();
    /// assert_eq!(Some(1), iter.next());
    /// assert_eq!(Some(&1), iter.last_item());
    /// assert_eq!(Some(2), iter.next());
    /// assert_eq!(Some(&2), iter.last_item());
    /// ```
    fn last_taken(self) -> LastTaken<Self>
    where
        Self: Sized,
    {
        LastTaken::new(self, None)
    }

    /// Creates an iterator that copies a slice of all its elements.
    ///
    /// # Panics
    /// Panics if the length of the slices is not equal.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let vec = {
    ///     let vec = vec![10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222];
    ///     let vec_of_slices: [&[u8]; 3] = [&vec[0..3], &vec[3..6], &vec[6..9]];
    ///     let iter = vec_of_slices.iter().copied().slice_copied::<3>();
    ///     iter.collect::<Vec<_>>()
    /// };
    /// assert_eq!(vec, vec![[10, 11, 22], [33, 44, 55], [66, 77, 88]]);
    /// ```
    fn slice_copied<const N: usize>(self) -> SliceCopied<Self, N>
    where
        Self: Sized + Clone,
    {
        SliceCopied::new(self)
    }

    /// An iterator adapter with combined ability to skip and step.
    ///
    /// # Panic
    /// The method panics when the specified step is 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let s = "iterator";
    /// let vec = s.chars().skip_step_by(3, 4).collect::<Vec<_>>();
    /// assert_eq!(vec, vec!['r', 'r']);
    /// let v = s.chars().skip(3).step_by(4).collect::<Vec<_>>();
    /// assert_eq!(v, vec!['r', 'r']);
    /// ```
    fn skip_step_by(self, skip: usize, step: usize) -> SkipStepBy<Self>
    where
        Self: Sized,
    {
        assert!(step != 0);
        SkipStepBy {
            iter: self.fuse(),
            skip,
            step: step - 1,
        }
    }

    /// The iterator adapter passing through the base iterator uses a closure at each step to change the number of steps.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let vec = vec![0u32, 11, 22, 33, 44, 55, 66, 77, 88, 99];
    /// let iter = vec.into_iter().step_by_fn(|s| {
    ///     if *s == 0 {
    ///         *s = 1;
    ///         1
    ///     } else {
    ///         *s += 2;
    ///         *s
    ///     }
    /// });
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, [0, 33, 88]);
    /// ```
    fn step_by_fn<F>(self, f: F) -> StepByFn<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut usize) -> usize,
    {
        StepByFn {
            iter: self.fuse(),
            f,
            skip: 0,
        }
    }
}

pub trait AllowZero {}

impl AllowZero for i8 {}
impl AllowZero for u8 {}
impl AllowZero for i16 {}
impl AllowZero for u16 {}
impl AllowZero for i32 {}
impl AllowZero for u32 {}
impl AllowZero for i64 {}
impl AllowZero for u64 {}
impl AllowZero for isize {}
impl AllowZero for usize {}
impl AllowZero for i128 {}
impl AllowZero for u128 {}
impl AllowZero for f32 {}
impl AllowZero for f64 {}
impl AllowZero for char {}
impl AllowZero for bool {}
impl<T: AllowZero, const N: usize> AllowZero for [T; N] {}

macro_rules! impl_allow_zero {
    ($($t:ident),*) => {
        impl<$($t: AllowZero),*> AllowZero for ($($t,)*) { }
    };

    ($($t:ident,)+) => {
        impl_allow_zero!($($t),*);

        impl<$($t: AllowZero),*> AllowZero for ($($t,)+) { }
    };
}

impl_allow_zero!(T);
impl_allow_zero!(S, T);
impl_allow_zero!(R, S, T);
impl_allow_zero!(Q, R, S, T);
impl_allow_zero!(N, Q, R, S, T);
impl_allow_zero!(M, N, Q, R, S, T);
impl_allow_zero!(L, M, N, Q, R, S, T);
impl_allow_zero!(K, L, M, N, Q, R, S, T);
impl_allow_zero!(J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(H, I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(G, H, I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(F, G, H, I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(E, F, G, H, I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(D, E, F, G, H, I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(C, D, E, F, G, H, I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(B, C, D, E, F, G, H, I, J, K, L, M, N, Q, R, S, T);
impl_allow_zero!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, Q, R, S, T);
