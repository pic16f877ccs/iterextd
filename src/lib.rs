#![crate_name = "iterextd"]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Adapters that extend [`Iterator`] functionality.
//!
//! ### Variable step:
//!
//! ```
//! use iterextd::IterExtd;
//!
//! let logic_fn = |s: &mut usize| {
//!     if *s == 0 {
//!         *s = 1;
//!     1
//!     } else {
//!         *s += 1;
//!         *s
//!     }
//! };
//! let iter = (0..18).step_by_fn(logic_fn);
//! let vec = iter.collect::<Vec<_>>();
//! assert_eq!(vec, vec![0, 2, 5, 9, 14]);
//! ```
//!
//! ### Collect a zeroed array:
//!
//! ```
//! use iterextd::IterExtd;
//!
//! let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//! let iter = arr.iter().filter(|elem| *elem % 2 == 0).copied();
//! let arr: (usize, [u8; 10]) = iter.collect_arr_zeroed();
//! assert_eq!(arr, (5, [2, 4, 6, 8, 10, 0, 0, 0, 0, 0]));
//! ```
//!
//! ### Collect windows from arrays:
//!
//! ```
//! use iterextd::IterExtd;
//!
//! let arr = [0, 1, 2, 3, 4, 5, 6, 7];
//! let iter = arr.into_iter();
//! let iter = iter.clone().map_iters(iter.previous(1).skip(2), |self_iter, arg_iter| {
//!     let (pre_elem, elem) = arg_iter.next()?;
//!     Some([self_iter.next()?, pre_elem, elem])
//! });
//! let vec = iter.collect::<Vec<_>>();
//! assert_eq!(vec, vec![[0, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5], [4, 5, 6], [5, 6, 7]]);
//! ```

mod gen_iterator;
#[doc = include_str!("../README.md")]
mod iterator;
mod slice_modify_iter;
mod structs;

pub use crate::gen_iterator::CircleBresenhamSeq;
pub use crate::iterator::IterExtd;
pub use crate::iterator::SwapIter;
pub use crate::iterator::TupleIntoIter;
pub use crate::iterator::TupleIter;
pub use crate::slice_modify_iter::GenRangeBounds;
pub use crate::slice_modify_iter::SliceModify;
pub use crate::slice_modify_iter::SliceModifyIter;
pub use crate::structs::ArrChunks;
pub use crate::structs::ArrayCloned;
pub use crate::structs::ArrayCopied;
pub use crate::structs::CombineIters;
pub use crate::structs::Extrapolate;
pub use crate::structs::GenCirclePoints;
pub use crate::structs::InclusiveStepBy;
pub use crate::structs::LastTaken;
pub use crate::structs::MapByThree;
pub use crate::structs::MapByTwo;
pub use crate::structs::MapIters;
pub use crate::structs::MissingIntegers;
pub use crate::structs::Offset;
pub use crate::structs::Previous;
pub use crate::structs::RangeIcvToTup;
pub use crate::structs::RangeToTup;
pub use crate::structs::SkipStepBy;
pub use crate::structs::SliceCopied;
pub use crate::structs::StepBoundary;
pub use crate::structs::StepByFn;
pub use crate::structs::TupToRange;
pub use crate::structs::TupToRangeIcv;
pub use crate::structs::TupleImut;
pub use crate::structs::TupleMut;

#[cfg(feature = "itern")]
#[cfg_attr(docsrs, doc(cfg(feature = "itern")))]
pub use crate::iterator::trait_itern::TupleItern;

use core::array::IntoIter;
use core::fmt::{self, Debug};
use core::hash::Hash;
use core::iter::{Fuse, FusedIterator};
use core::marker::PhantomData;
use core::mem::{swap, MaybeUninit};
use core::ops::{Add, AddAssign, Deref, Range, RangeInclusive, Sub};
use core::ptr;
use core::slice::SliceIndex;
use fixedbitset::{FixedBitSet, IntoOnes};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use num::Zero;
use num_convert::{ToZero, TryFromByAdd};
use num_integer::{gcd, Integer};
use std::collections::HashMap;
