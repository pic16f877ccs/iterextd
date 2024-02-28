#![crate_name = "iterextd"]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]

//! Adapters that extend [`Iterator`] functionality.
//!
//! ### Variable step
//!
//! ```
//! use iterextd::IterExtd;
//!
//! let logic_fn = |s: &mut usize| { if *s == 0 { *s = 1; 1 } else { *s += 1; *s } };
//! let iter = (0..18).step_by_fn(logic_fn);
//! let vec = iter.collect::<Vec<_>>();
//! assert_eq!(vec, vec![0, 2, 5, 9, 14]);
//! ```
//!
//! ### Collect a zeroed array
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
//! ### Collect windows from arrays.
//!
//! ```
//! use iterextd::IterExtd;
//!
//!  let arr = [0, 1, 2, 3, 4, 5, 6, 7];
//!  let iter = arr.into_iter();
//!  let iter = iter.clone().map_iters(iter.previous(1).skip(2), |self_iter, arg_iter| {
//!      let (pre_elem, elem) = arg_iter.next()?; Some([self_iter.next()?, pre_elem, elem]) });
//!  let vec = iter.collect::<Vec<_>>();
//!  assert_eq!(vec, vec![[0, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5], [4, 5, 6], [5, 6, 7]]);
//! ```
mod iterator;
mod structs;

pub use crate::iterator::IterExtd;
pub use crate::iterator::TupleIter;
pub use crate::iterator::TupleIntoIter;
pub use crate::structs::ArrChunks;
pub use crate::structs::ArrayCloned;
pub use crate::structs::ArrayCopied;
pub use crate::structs::CombineIters;
pub use crate::structs::LastTaken;
pub use crate::structs::MapByThree;
pub use crate::structs::MapByTwo;
pub use crate::structs::MapIters;
pub use crate::structs::Previous;
pub use crate::structs::SkipStepBy;
pub use crate::structs::SliceCopied;
pub use crate::structs::StepByFn;
pub use crate::structs::TupleImut;
pub use crate::structs::TupleMut;

#[cfg(feature = "itern")]
pub use crate::iterator::trait_itern::TupleItern;

use core::fmt;
use core::iter::{Fuse, FusedIterator};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ptr;
use core::array::IntoIter;
