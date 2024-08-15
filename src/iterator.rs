use crate::structs::{
    ArrChunks, ArrayCloned, ArrayCopied, CombineIters, Extrapolate, InclusiveStepBy, LastTaken,
    MapByThree, MapByTwo, MapIters, MissingIntegers, Offset, Previous, RangeIcvToTup, RangeToTup,
    SkipStepBy, SliceCopied, StepBoundary, StepByFn, TupToRange, TupToRangeIcv, TupleImut,
    TupleMut, UniqueSorted,
};
use crate::swap;
use crate::Debug;
use crate::FixedBitSet;
use crate::FusedIterator;
use crate::IntoIter;
use crate::Itertools;
use crate::MaybeUninit;
use crate::MinMax;
use crate::PhantomData;
use crate::Zero;
use crate::{gcd, Integer};
use crate::{Deref, Range};
use crate::{ToZero, TryFromByAdd};

#[cfg(feature = "std")]
use crate::Hash;
#[cfg(feature = "std")]
use crate::HashMap;

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
    ///
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
    fn array_cloned<'a, T, const N: usize>(self) -> ArrayCloned<Self, N>
    where
        Self: Sized + Iterator<Item = [&'a T; N]>,
        T: Clone + 'a,
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
    fn array_copied<'a, T, const N: usize>(self) -> ArrayCopied<Self, N>
    where
        Self: Sized + Iterator<Item = [&'a T; N]>,
        T: Copy + 'a,
    {
        ArrayCopied::new(self)
    }

    /// Collect a zeroed array for nullable types.
    ///
    /// # Panics
    ///
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
    ///
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
    fn consume(self)
    where
        Self: Sized,
    {
        for _ in self {}
    }

    /// Combine two iterators in parts sequentially.
    /// The length of the piece can be set for each separately.
    ///
    /// # Panics
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

    /// Counts the frequency of each element in the iterator.
    ///
    /// # Warning
    ///
    /// The results are stored in a `HashMap`, which does not preserve the order of the elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let vec = vec![1, 2, 2, 3, 3, 3];
    /// let mut freqs = vec.into_iter().count_freq().collect::<Vec<_>>();
    /// freqs.sort();
    /// assert_eq!(freqs, vec![(1, 1), (2, 2), (3, 3)]);
    /// ```
    #[cfg(feature = "std")]
    fn count_freq(self) -> impl Iterator<Item = (Self::Item, usize)> + Debug
    where
        Self: Sized,
        Self::Item: Eq + Hash + Debug,
    {
        let mut hash_map = HashMap::new();
        for item in self {
            hash_map
                .entry(item)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        hash_map.into_iter()
    }

    /// Extrapolates the iterator's elements.
    ///
    /// # Panics
    ///
    /// This method will panic in debug mode if the extrapolated values cause a computation overflow.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr: [f32; 2] = [0.00, 0.01];
    /// let extrapolated = arr.into_iter().extrapolate().take(5).collect::<Vec<_>>();
    /// assert_eq!(extrapolated, vec![0.0, 0.01, 0.02, 0.03, 0.04]);
    ///
    /// let vec = vec![2, 5, 6, 9, 13];
    /// let extrapolated: Vec<_> = vec.into_iter().extrapolate().take(10).collect();
    /// assert_eq!(extrapolated, vec![2, 5, 6, 9, 13, 17, 21, 25, 29, 33]);
    /// ```
    fn extrapolate(self) -> Extrapolate<Self>
    where
        Self: Sized,
        Self::Item: Zero,
    {
        Extrapolate {
            iter: self,
            arg_one: Self::Item::zero(),
            arg_two: Self::Item::zero(),
        }
    }

    /// Finds the greatest common divisor (GCD) of the elements in the iterator.
    ///
    /// # Notes
    ///
    /// - **NOTE:** This method will return `None` if the iterator is empty.
    /// - **NOTE:** The minimum sequence length must be two elements; otherwise, `None` will be returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let vec = vec![24, 36, 48];
    /// assert_eq!(vec.iter().gcd(), Some(12));
    ///
    /// let empty_vec: Vec<i32> = vec![];
    /// assert_eq!(empty_vec.iter().gcd(), None);
    ///
    /// let single_element = vec![42];
    /// assert_eq!(single_element.iter().gcd(), None);
    /// ```
    fn gcd(mut self) -> Option<<<Self as Iterator>::Item as Deref>::Target>
    where
        Self: Sized,
        <Self as Iterator>::Item: Deref,
        <<Self as Iterator>::Item as Deref>::Target: Integer + Copy,
    {
        let acc = gcd(*self.next()?, *self.next()?);
        Some(self.fold(acc, |acc, x| gcd(acc, *x)))
    }

    /// Create an indexes iterator with a start and end for each step.
    ///
    /// With each iteration, it furnishes the start and end indices of the current step,
    /// while also accounting for any residual elements in the last incomplete step.
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
    /// use iterextd::IterExtd;
    ///
    /// let s = "slice index iterator";
    /// let iter = (0..s.len()).step_boundary(3);
    /// iter.for_each(|bounds| { println!("{:?}", &s[bounds.0..=bounds.1] ); });
    /// ```
    fn step_boundary(self, size: usize) -> StepBoundary<Self>
    where
        Self: Sized,
    {
        assert!(size != 0);
        StepBoundary::new(self, size)
    }

    /// Creates an iterator that performs the given step at each iteration, returning inclusive of the first and last element.
    ///
    /// If there are elements smaller than the remaining step, the last element will be returned.
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
    /// use iterextd::IterExtd;
    ///
    /// let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let iter = arr.into_iter().inclusive_step_by(4);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![0, 4, 8, 9]);
    /// ```
    fn inclusive_step_by(self, step: usize) -> InclusiveStepBy<Self>
    where
        Self: Sized,
    {
        assert!(step != 0);
        InclusiveStepBy::new(self, step)
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

    /// Return an iterator adapter that yields missing integers.
    /// Note that the maximum value of an iterator element must be less than or equal to
    /// `usize::MAX` for unsigned type and `isize::MAX for signed`!
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [9u8, 5, 6, 4, 8, 8, 2, 4, 10, 2, 12];
    /// let vec = arr.iter().missing_integers().collect::<Vec<_>>();
    /// assert_eq!(vec, vec![3, 7, 11]);
    /// ```
    fn missing_integers(mut self) -> MissingIntegers<Self>
    where
        Self: Iterator + Sized + Clone,
        <Self as Iterator>::Item: PartialOrd + Deref,
        <<Self as Iterator>::Item as Deref>::Target: Copy,
        usize: TryFromByAdd<<<Self as Iterator>::Item as Deref>::Target>,
    {
        let (min, size) = match self.clone().minmax() {
            MinMax(min, max) => {
                let min_max = (<usize as TryFromByAdd<<<Self as Iterator>::Item as Deref>::Target>>::try_from_by_add(*min),
                    <usize as TryFromByAdd<<<Self as Iterator>::Item as Deref>::Target>>::try_from_by_add(*max));
                match min_max {
                    (Some(min), Some(max)) => (min, max - min + 1),
                    _ => return MissingIntegers::default(),
                }
            }
            _ => return MissingIntegers::default(),
        };

        let mut bitset = FixedBitSet::with_capacity(size);
        bitset.toggle_range(..);

        match self.try_for_each(|x| {
            if let Some(val) = <usize as TryFromByAdd<
                <<Self as Iterator>::Item as Deref>::Target,
            >>::try_from_by_add(*x)
            {
                bitset.remove(val - min);
                Some(())
            } else {
                None
            }
        }) {
            Some(_) => MissingIntegers {
                iter: bitset.into_ones(),
                min,
                _phantom: PhantomData,
            },
            _ => MissingIntegers::default(),
        }
    }

    /// Returns an iterator adapter that finds missing integers.
    /// This adapter must use a unique and sorted iterator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let mut vec = vec![-2i8, 2, 4, 5, 6, 8, 9, 10, 12];
    /// let mut iter = vec.iter().missing_integers_uqsort();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![-1, 0, 1, 3, 7, 11]);
    /// ```
    fn missing_integers_uqsort(
        self,
    ) -> impl Iterator<Item = <Range<<<Self as Iterator>::Item as Deref>::Target> as Iterator>::Item>
           + Debug
    where
        Self: Iterator + Sized + Clone + Debug,
        <Self as Iterator>::Item: PartialOrd + Deref + Debug,
        <<Self as Iterator>::Item as Deref>::Target:
            Copy + Debug + ToZero<<<Self as Iterator>::Item as Deref>::Target>,
        Range<<<Self as Iterator>::Item as Deref>::Target>: Iterator,
        <Range<<<Self as Iterator>::Item as Deref>::Target> as Iterator>::Item:
            PartialOrd<<<Self as Iterator>::Item as Deref>::Target>,
    {
        let (min, max) = match self.clone().minmax() {
            MinMax(min, max) => (*min, *max),
            _ => (
                <<Self as Iterator>::Item as Deref>::Target::to_zero(),
                <<Self as Iterator>::Item as Deref>::Target::to_zero(),
            ),
        };

        (min..max).map_iters(self.peekable(), |range_it, seq_it| {
            for range_val in range_it.by_ref() {
                if range_val >= **seq_it.peek()? {
                    seq_it.next();
                    continue;
                } else {
                    return Some(range_val);
                };
            }
            None
        })
    }

    /// Finds the mode(s) of the iterator's elements.
    ///
    /// # Warning
    ///
    /// The results are stored in a `HashMap`, which does not preserve the order of the elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let vec = vec![1, 2, 2, 3, 3, 3, 3];
    /// let modes = vec.into_iter().modes().collect::<Vec<_>>();
    /// assert_eq!(modes, vec![(3, 4)]);
    /// ```
    #[cfg(feature = "std")]
    fn modes(self) -> impl Iterator<Item = (Self::Item, usize)> + Debug
    where
        Self: Sized,
        Self::Item: Eq + Hash + Debug,
    {
        let mut hash_map = HashMap::new();
        let mut max: usize = 0;
        for item in self {
            let count = hash_map.entry(item).or_insert(0);
            *count += 1;
            if *count > max {
                max = *count;
            }
        }

        hash_map.into_iter().filter(move |&(_, count)| count == max)
    }

    /// The iterator adapter adds an offset to a two-element tuple.
    ///
    /// # Warning
    ///
    /// When using this iterator adapter, the output value may overflow.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::{CircleBresenhamSeq, IterExtd};
    ///
    /// let radius = 1_u8;
    /// let center_x = radius as i32;
    /// let center_y = radius as i32;
    /// let iter = CircleBresenhamSeq::<i32>::new(radius);
    /// let circle_with_offset = iter.offset(center_x, center_y);
    /// let vec = circle_with_offset.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(1, 0), (2, 1), (1, 2), (0, 1)]);
    /// ```
    #[inline]
    fn offset<T>(self, offset_x: T, offset_y: T) -> Offset<T, Self>
    where
        Self: Sized + Iterator,
    {
        Offset {
            iter: self,
            offset_x,
            offset_y,
        }
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
    ///
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

    /// An iterator adapter with the combined ability to skip and step.
    ///
    /// # Panics
    ///
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

    /// Return an iterator that converts a tuple at each iteration to a [RangeInclusive](crate::RangeInclusive).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [(0, 2), (3, 5), (6, 8),];
    /// let iter = arr.iter().cloned().to_range_icv();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![0..=2, 3..=5, 6..=8]);
    /// ```
    fn to_range_icv(self) -> TupToRangeIcv<Self>
    where
        Self: Sized,
    {
        TupToRangeIcv { iter: self }
    }

    /// Return an iterator that converts a tuple at each iteration to a [Range](crate::Range).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [(0, 2), (3, 5), (6, 8),];
    /// let iter = arr.iter().cloned().to_range();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![0..2, 3..5, 6..8]);
    /// ```
    fn to_range(self) -> TupToRange<Self>
    where
        Self: Sized,
    {
        TupToRange { iter: self }
    }

    /// Return an iterator that converts a [Range](crate::Range) at each iteration to a tuple.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [0..2, 3..5, 6..8];
    /// let iter = arr.iter().cloned().to_tuple();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(0, 2), (3, 5), (6, 8),]);
    /// ```
    fn to_tuple(self) -> RangeToTup<Self>
    where
        Self: Sized,
    {
        RangeToTup { iter: self }
    }

    /// Return an iterator that converts a [RangeInclusive](crate::RangeInclusive) at each iteration to a tuple.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [0..=2, 3..=5, 6..=8];
    /// let iter = arr.iter().cloned().to_tuple_icv();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(0, 2), (3, 5), (6, 8),]);
    /// ```
    fn to_tuple_icv(self) -> RangeIcvToTup<Self>
    where
        Self: Sized,
    {
        RangeIcvToTup { iter: self }
    }

    /// Return an iterator adapter that yields unique sorted integers.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::IterExtd;
    ///
    /// let arr = [9u8, 5, 6, 4, 8, 8, 2, 4, 10, 2, 12];
    /// let vec = arr.iter().unique_sorted().collect::<Vec<_>>();
    /// assert_eq!(vec, vec![2, 4, 5, 6, 8, 9, 10, 12]);
    /// ```
    fn unique_sorted(mut self) -> UniqueSorted<Self>
    where
        Self: Iterator + Sized + Clone,
        <Self as Iterator>::Item: PartialOrd + Deref,
        <<Self as Iterator>::Item as Deref>::Target: Copy,
        usize: TryFromByAdd<<<Self as Iterator>::Item as Deref>::Target>,
    {
        let (min, size) = match self.clone().minmax() {
            MinMax(min, max) => {
                let min_max = (<usize as TryFromByAdd<<<Self as Iterator>::Item as Deref>::Target>>::try_from_by_add(*min),
                    <usize as TryFromByAdd<<<Self as Iterator>::Item as Deref>::Target>>::try_from_by_add(*max));
                match min_max {
                    (Some(min), Some(max)) => (min, max - min + 1),
                    _ => return UniqueSorted::default(),
                }
            }
            _ => return UniqueSorted::default(),
        };

        let mut bitset = FixedBitSet::with_capacity(size);

        match self.try_for_each(|x| {
            if let Some(val) = <usize as TryFromByAdd<
                <<Self as Iterator>::Item as Deref>::Target,
            >>::try_from_by_add(*x)
            {
                bitset.insert(val - min);
                Some(())
            } else {
                None
            }
        }) {
            Some(_) => UniqueSorted {
                iter: bitset.into_ones(),
                min,
                _phantom: PhantomData,
            },
            _ => UniqueSorted::default(),
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

/// A trait that allows iterating over a tuple.
pub trait TupleIter<'a> {
    /// The type of immutable references yielded by the tuple iterator.
    type TupImut;
    /// The type of mutable references yielded by the tuple iterator.
    type TupMut;

    /// A tuple iterator that allows you to get each value by reference.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::TupleIter;
    ///
    /// let tup = (3, 2, 1);
    /// let mut tup_iter = tup.tuple_iter().rev();
    /// assert_eq!(tup_iter.next(), Some(&1));
    /// assert_eq!(tup_iter.next(), Some(&2));
    /// assert_eq!(tup_iter.next(), Some(&3));
    /// assert_eq!(tup_iter.next(), None);
    /// ```
    fn tuple_iter(&'a self) -> Self::TupImut;

    /// A tuple iterator that allows you to get each value by mutable reference.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::TupleIter;
    ///
    /// let mut tup = (vec![20], vec![21], vec![22], vec![23]);
    /// let _ = tup.tuple_iter_mut().for_each(|elem| { elem[0] +=10;});
    /// assert_eq!(tup, (vec![30], vec![31], vec![32], vec![33]));
    /// ```
    fn tuple_iter_mut(&'a mut self) -> Self::TupMut;
}

impl<'a, T, const N: usize> Iterator for TupleImut<'a, T, N> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.arr[self.idx_iter.next()?])
    }
}

impl<'a, T, const N: usize> Iterator for TupleMut<'a, T, N> {
    type Item = &'a mut T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe { self.arr[self.idx_iter.next()?].as_mut() }
    }
}

impl<'a, T, const N: usize> DoubleEndedIterator for TupleMut<'a, T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe { self.arr[self.idx_iter.next_back()?].as_mut() }
    }
}

impl<'a, T, const N: usize> DoubleEndedIterator for TupleImut<'a, T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(self.arr[self.idx_iter.next_back()?])
    }
}

impl<'a, T, const N: usize> ExactSizeIterator for TupleMut<'a, T, N> {
    #[inline]
    fn len(&self) -> usize {
        self.idx_iter.len()
    }
}
impl<'a, T, const N: usize> ExactSizeIterator for TupleImut<'a, T, N> {
    #[inline]
    fn len(&self) -> usize {
        self.idx_iter.len()
    }
}

macro_rules! impl_tuple_iter {
    ($N:tt; $($n:tt $t:tt),+) => {
        impl<'a, T: 'a> TupleIter<'a> for ($($t,)+)
            {
                type TupImut = TupleImut<'a, T, $N>;
                type TupMut = TupleMut<'a, T, $N>;

                #[inline]
                fn tuple_iter(&'a self) -> Self::TupImut {
                    TupleImut {
                        arr: [$(&self.$n,)+],
                        idx_iter: 0..$N,
                    }
                }

                #[inline]
                fn tuple_iter_mut(&'a mut self) -> Self::TupMut {
                    TupleMut {
                        arr: [$(&mut self.$n as *mut T,)+],
                        idx_iter: 0..$N,
                        _unused: PhantomData,
                    }
                }
            }
    };
}

impl_tuple_iter!(1; 0 T);
impl_tuple_iter!(2; 0 T, 1 T);
impl_tuple_iter!(3; 0 T, 1 T, 2 T);
impl_tuple_iter!(4; 0 T, 1 T, 2 T, 3 T);
impl_tuple_iter!(5; 0 T, 1 T, 2 T, 3 T, 4 T);
impl_tuple_iter!(6; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T);
impl_tuple_iter!(7; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T);
impl_tuple_iter!(8; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T);
impl_tuple_iter!(9; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T);
impl_tuple_iter!(10; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T);
impl_tuple_iter!(11; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T, 10 T);
impl_tuple_iter!(12; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T, 10 T, 11 T);

#[cfg(feature = "itern")]
#[cfg_attr(docsrs, doc(cfg(feature = "itern")))]
pub mod trait_itern {
    use crate::structs::{TupleImut, TupleMut};
    use crate::PhantomData;

    /// A trait that allows iteration over N elements of a tuple.
    pub trait TupleItern<'a> {
        /// The type of immutable references yielded by the tuple iterator.
        type A;
        /// The type of mutable references yielded by the tuple iterator.
        type B;

        /// Creates an iterator from a tuple that returns elements by reference.
        ///
        /// The difference from the [tuple_iter](crate::TupleIter::tuple_iter) method is that it can get N elements.
        ///
        /// # Panics
        ///
        /// If N is greater than the number of elements in the tuple, panic.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use iterextd::TupleItern;
        ///
        /// let tup = (5, 4, 3, 2, 1);
        /// let mut tup_iter = tup.tuple_itern::<3>().rev();
        /// assert_eq!(tup_iter.next(), Some(&3));
        /// assert_eq!(tup_iter.next(), Some(&4));
        /// assert_eq!(tup_iter.next(), Some(&5));
        /// assert_eq!(tup_iter.next(), None);
        /// ```
        fn tuple_itern<const N: usize>(&'a self) -> TupleImut<'a, Self::A, N>;

        /// Creates an iterator from a tuple that returns elements by mutable reference.
        ///
        /// The difference from the [tuple_iter_mut](crate::TupleIter::tuple_iter_mut) method is that it can get N elements.
        ///
        /// # Panics
        ///
        /// If N is greater than the number of elements in the tuple, panic.
        ///
        /// # Examples
        ///
        /// Basic usage:
        ///
        /// ```
        /// use iterextd::TupleItern;
        ///
        /// let mut tup = (vec![20], vec![21], vec![22], vec![23]);
        /// let _ = tup.tuple_itern_mut::<2>().for_each(|elem| { elem[0] +=10;});
        /// assert_eq!(tup, (vec![30], vec![31], vec![22], vec![23]));
        /// ```
        fn tuple_itern_mut<const N: usize>(&'a mut self) -> TupleMut<'a, Self::B, N>;
    }

    macro_rules! impl_tuple_itern {
        ($($n:tt $t:tt),+) => {
            impl<'a, T> TupleItern<'a> for ($($t,)+)
                {
                    type A = T;
                    type B = T;
                    fn tuple_itern<const N: usize>(&'a self) -> TupleImut<'_, Self::A, N> {
                        TupleImut {
                            arr: core::array::from_fn(|i| match i {
                                $($n => &self.$n,)+
                                _ => panic!(),
                            }),
                            idx_iter: 0..N,
                        }
                    }

                    fn tuple_itern_mut<const N: usize>(&'a mut self) -> TupleMut<'_, Self::B, N> {
                        TupleMut {
                            arr: core::array::from_fn(|i| match i {
                                $($n => &mut self.$n as *mut T,)+
                                _ => panic!(),
                            }),
                            idx_iter: 0..N,
                            _unused: PhantomData,
                        }
                    }
                }
        };
    }

    impl_tuple_itern!(0 T);
    impl_tuple_itern!(0 T, 1 T);
    impl_tuple_itern!(0 T, 1 T, 2 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T, 5 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T, 10 T);
    impl_tuple_itern!(0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T, 10 T, 11 T);
}

/// Tuple iterator, adds the ability to get elements by value.
pub trait TupleIntoIter<T, const N: usize> {
    /// Creates an iterator from a tuple that returns elements by value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::{TupleIter, TupleIntoIter};
    ///
    /// let tup = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    /// let iter = tup.tuple_iter();
    /// let vec = iter.flat_map(|elem|{ elem.tuple_into_iter() }).collect::<Vec<_>>();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    fn tuple_into_iter(self) -> IntoIter<T, N>;
}

macro_rules! impl_tuple_into_iter {
    ($N:tt; $($n:tt $t:tt),+) => {
        impl<T> TupleIntoIter<T, $N> for ($($t,)+) {
            fn tuple_into_iter(self) -> IntoIter<T, $N> {
                [$(self.$n,)+].into_iter()
            }
        }
    };
}

impl_tuple_into_iter!(1;  0 T);
impl_tuple_into_iter!(2;  0 T, 1 T);
impl_tuple_into_iter!(3;  0 T, 1 T, 2 T);
impl_tuple_into_iter!(4;  0 T, 1 T, 2 T, 3 T);
impl_tuple_into_iter!(5;  0 T, 1 T, 2 T, 3 T, 4 T);
impl_tuple_into_iter!(6;  0 T, 1 T, 2 T, 3 T, 4 T, 5 T);
impl_tuple_into_iter!(7;  0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T);
impl_tuple_into_iter!(8;  0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T);
impl_tuple_into_iter!(9;  0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T);
impl_tuple_into_iter!(10; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T);
impl_tuple_into_iter!(11; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T, 10 T);
impl_tuple_into_iter!(12; 0 T, 1 T, 2 T, 3 T, 4 T, 5 T, 6 T, 7 T, 8 T, 9 T, 10 T, 11 T);

impl<T: ?Sized> SwapIter<'_> for T where T: Iterator {}

/// An iterator adapter that swaps elements in two sequences.
pub trait SwapIter<'a>: Iterator {
    /// Swap elements between two mutable iterators.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iterextd::SwapIter;
    ///
    /// let mut first_vec = [0, 1, 2, 3, 4, 5, 6, 7];
    /// let mut second_vec = vec![10, 11, 12, 13, 14, 15];
    /// let first_iter = first_vec.iter_mut().step_by(2);
    /// let second_iter = second_vec.iter_mut().step_by(2);
    /// let _ = first_iter.swap_elems(second_iter);
    /// assert_eq!(first_vec, [10, 1, 12, 3, 14, 5, 6, 7]);
    /// assert_eq!(second_vec, [0, 11, 2, 13, 4, 15]);
    /// ```
    fn swap_elems<I, T: 'a>(self, mut other_iter: I)
    where
        I: Iterator<Item = &'a mut T>,
        Self: Sized + Iterator<Item = &'a mut T>,
    {
        for self_elem in self {
            let Some(other_elem) = other_iter.next() else {
                break;
            };
            swap(self_elem, other_elem);
        }
    }
}
