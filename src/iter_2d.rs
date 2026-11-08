use crate::FusedIterator;

impl<T: Iterator + ?Sized> Iter2D for T {}

impl<I, J, T> FusedIterator for Overlay2D<I, J, T>
where
    I: FusedIterator<Item = T>,
    J: Iterator<Item = T>,
{
}

impl<I, J, T> ExactSizeIterator for Overlay2D<I, J, T>
where
    I: ExactSizeIterator<Item = T>,
    J: Iterator<Item = T>,
{
    #[inline]
    fn len(&self) -> usize {
        self.base_iter.len()
    }
}

/// A trait for 2D operations on flat iterators.
///
/// This trait enables working with abstract 2D data structures using regular 1D iterators.
/// It is especially useful for overlaying or manipulating rectangular regions within a flat data buffer.
pub trait Iter2D: Iterator {
    /// Overlays a 2D region from another iterator (`overlay_iter`) onto the base iterator (`self`)
    /// at the specified offset and size within the target 2D area.
    ///
    /// # Arguments
    /// * `overlay_iter` - Iterator providing the elements to overlay.
    /// * `target_size` - Size of the base 2D area as (width, height).
    /// * `overlay_size` - Size of the overlay region as (width, height).
    /// * `overlay_offset` - Offset (x, y) in the base area where the overlay region should be inserted.
    ///
    /// # Panics
    /// Panics if the overlay region does not fit within the target area, if any size
    /// is zero, or if the target area cannot be represented as a `usize`.
    ///
    /// # Returns
    /// Returns an `Overlay2D` iterator that yields elements from the base iterator,
    /// with the specified region replaced by elements from `overlay_iter`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iterextd::Iter2D;
    /// let b_vec = vec!['□'; 36];
    /// let o_vec = vec!['■'; 16];
    ///
    /// let vec_b_o = b_vec
    ///     .into_iter()
    ///     .overlay_2d(o_vec.into_iter(), (6, 6), (4, 4), (1, 1))
    ///     .collect::<Vec<_>>();
    ///
    /// let vec = vec_b_o.chunks(6).collect::<Vec<_>>();
    ///
    /// let vec_overlay = vec![
    ///     ['□', '□', '□', '□', '□', '□'],
    ///     ['□', '■', '■', '■', '■', '□'],
    ///     ['□', '■', '■', '■', '■', '□'],
    ///     ['□', '■', '■', '■', '■', '□'],
    ///     ['□', '■', '■', '■', '■', '□'],
    ///     ['□', '□', '□', '□', '□', '□'],
    /// ];
    ///
    /// assert_eq!(vec, vec_overlay);
    /// ```
    #[inline]
    fn overlay_2d<J, T>(
        self,
        overlay_iter: J,
        target_size: (usize, usize),
        overlay_size: (usize, usize),
        overlay_offset: (usize, usize),
    ) -> Overlay2D<Self, J, T>
    where
        Self: Sized + Iterator<Item = T>,
        J: Iterator<Item = T>,
    {
        if (target_size.0 == 0)
            || (target_size.1 == 0)
            || (overlay_size.0 == 0)
            || (overlay_size.1 == 0)
        {
            panic!("the size of abstract 2D data must not be zero");
        }

        let overlay_right_edge = overlay_offset.0.checked_add(overlay_size.0);
        let overlay_bottom_edge = overlay_offset.1.checked_add(overlay_size.1);
        if overlay_right_edge.is_none_or(|edge| edge > target_size.0)
            || overlay_bottom_edge.is_none_or(|edge| edge > target_size.1)
        {
            panic!("the offset and size of the inserted abstract 2D data exceed the bounds");
        }
        let total_target = target_size
            .0
            .checked_mul(target_size.1)
            .expect("the size of abstract 2D data exceeds usize");

        let overlay_start_index = target_size
            .0
            .checked_mul(overlay_offset.1)
            .and_then(|row| row.checked_add(overlay_offset.0))
            .expect("overlay start index exceeds usize");
        let total_overlay = overlay_size
            .0
            .checked_mul(overlay_size.1)
            .expect("the size of abstract 2D data exceeds usize");

        Overlay2D {
            base_iter: self,
            overlay_iter,
            base_index: 0,
            overlay_index: overlay_start_index,
            overlay_row_width: if target_size.0 == overlay_size.0 {
                total_overlay
            } else {
                overlay_size.0
            },
            overlay_row_padding: if target_size.0 == overlay_size.0 {
                total_target
                    .checked_sub(
                        total_overlay
                            .checked_add(overlay_start_index)
                            .expect("overlay end index exceeds usize"),
                    )
                    .expect("overlay end index exceeds target area")
            } else {
                target_size.0 - overlay_size.0
            },
            in_overlay_row: false,
        }
    }
}

/// Iterator adapter for overlaying one 2D sequence onto another.
///
/// Allows inserting elements from `overlay_iter` into the base iterator `base_iter`
/// at a specified offset and size. Useful for abstract 2D structures represented as 1D iterators.
#[derive(Debug, Clone)]
pub struct Overlay2D<I: Iterator<Item = T>, J: Iterator<Item = T>, T> {
    base_iter: I,
    overlay_iter: J,
    base_index: usize,
    overlay_index: usize,
    overlay_row_width: usize,
    overlay_row_padding: usize,
    in_overlay_row: bool,
}

impl<I, J, T> Iterator for Overlay2D<I, J, T>
where
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let iter_item = self.base_iter.next();

        // usize::MAX marks the end of the overlay; it is not a valid target index.
        if self.overlay_index == usize::MAX {
            return iter_item;
        }

        if self.base_index == self.overlay_index {
            if self.in_overlay_row {
                self.in_overlay_row = false;
                self.overlay_index = self.overlay_index.saturating_add(self.overlay_row_padding);
            } else {
                self.in_overlay_row = true;
                self.base_index = self.base_index.saturating_add(self.overlay_row_width);
            }
        }

        if self.base_index > self.overlay_index {
            self.overlay_index = self.overlay_index.saturating_add(1);
            let iter_other_item = self.overlay_iter.next();
            if iter_other_item.is_none() || iter_item.is_none() {
                self.overlay_index = usize::MAX;
                return iter_item;
            }
            return iter_other_item;
        }
        self.base_index = self.base_index.saturating_add(1);

        iter_item
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base_iter.size_hint()
    }
}
