use crate::Debug;
use crate::FusedIterator;
use crate::{Add, AddAssign, Sub};
use core::ops::{Mul, Neg};
use num::Unsigned;

/// An iterator for creating a circle sequentially in a clockwise direction.
#[derive(Debug, Clone)]
pub struct CircleBresenhamSeq<T> {
    x: T,
    y: T,
    err: T,
    i: u8,
}

impl<T> CircleBresenhamSeq<T>
where
    T: From<i8> + PartialEq + Neg<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    /// Create a new instance of `CircleBresenhamSeq`.
    ///
    /// # Arguments
    ///
    /// * `radius` - unsigned iteger, radius of the generated circle points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use iterextd::CircleBresenhamSeq;
    ///
    /// let radius = 1_u8;
    /// let iter = CircleBresenhamSeq::<i16>::new(radius);
    /// let circle = iter.collect::<Vec<_>>();
    /// assert_eq!(circle, vec![(0, -1), (1, 0), (0, 1), (-1, 0)]);
    /// ```
    #[inline]
    pub fn new<U>(radius: U) -> Self
    where
        U: Copy + Unsigned + From<u8> + PartialOrd,
        T: From<U>,
    {
        if radius > U::from(0) {
            return Self {
                x: -T::from(radius),
                y: 0.into(),
                err: T::from(2) - T::from(2) * T::from(radius),
                i: 1,
            };
        }

        Self::default()
    }
}

impl<T> Default for CircleBresenhamSeq<T>
where
    T: From<i8>,
{
    /// Create a default instance of `CircleBresenhamSeq`.
    #[inline]
    fn default() -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
            err: 0.into(),
            i: 4,
        }
    }
}

impl<T> Iterator for CircleBresenhamSeq<T>
where
    T: Add<Output = T>
        + From<i8>
        + Neg<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Copy
        + PartialEq
        + PartialOrd
        + AddAssign,
{
    type Item = (T, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == T::from(0) {
            self.i += 1;
            if self.i == 5 {
                return None;
            }

            self.x = -self.y;
            self.err = T::from(2) - T::from(2) * self.y;
            self.y = T::from(0);
        }

        let r = self.err;
        let xy = if self.i == 1 {
            (self.y, self.x)
        } else if self.i == 2 {
            (-self.x, self.y)
        } else if self.i == 3 {
            (-self.y, -self.x)
        } else {
            (self.x, -self.y)
        };

        if r <= self.y {
            self.y += T::from(1);
            self.err += T::from(2) * self.y + T::from(1);
        }
        if r > self.x || self.err > self.y {
            self.x += T::from(1);
            self.err += T::from(2) * self.x + T::from(1);
        }

        Some(xy)
    }
}

impl<T> DoubleEndedIterator for CircleBresenhamSeq<T>
where
    T: Add<Output = T>
        + From<i8>
        + Neg<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Copy
        + PartialOrd
        + AddAssign,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x == T::from(0) {
            self.i += 1;
            if self.i == 5 {
                return None;
            }

            self.x = -self.y;
            self.err = T::from(2) - T::from(2) * self.y;
            self.y = T::from(0);
        }

        let xy = if self.i == 1 {
            (-self.y, self.x)
        } else if self.i == 2 {
            (self.x, self.y)
        } else if self.i == 3 {
            (self.y, -self.x)
        } else {
            (-self.x, -self.y)
        };

        let r = self.err;
        if r <= self.y {
            self.y += T::from(1);
            self.err += T::from(2) * self.y + T::from(1);
        }
        if r > self.x || self.err > self.y {
            self.x += T::from(1);
            self.err += T::from(2) * self.x + T::from(1);
        }

        Some(xy)
    }
}

impl<T> FusedIterator for CircleBresenhamSeq<T> where
    T: Add<Output = T>
        + From<i8>
        + Neg<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Copy
        + PartialOrd
        + AddAssign
{
}
