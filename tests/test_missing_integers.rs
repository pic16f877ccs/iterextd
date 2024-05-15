#[cfg(test)]
mod tests {
    use iterextd::IterExtd;
    use itertools::Itertools;

    #[test]
    fn test_missing_int_signed() {
        let vec = vec![9i8, -8, -8, -5, -4, -3, -2, 0, 1, 2, 4, 5, 6, 8, 10];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![-7, -6, -1, 3, 7]);

        let arr = &[9i8, -8, -8, -5, -4, -3, -2, 0, 1, 2, 4, 5, 6, 8, 10];
        assert_eq!(arr.iter().missing_integers().collect::<Vec<_>>(), vec![-7, -6, -1, 3, 7]);

        let vec = vec![-3, 3];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![-2, -1, 0, 1, 2]);

        let a = isize::MAX as i128;
        let b = a - 1;
        let c = a - 3;
        let d = a + 1;

        let vec = vec![a, b, c];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![a - 2]);

        let vec = vec![a, b, c, d];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![]);

        let vec = vec![4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 8];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![]);

        let arr = [0i8; 0];
        assert_eq!(arr.iter().missing_integers().collect::<Vec<_>>(), vec![]);

        let arr = [0];
        assert_eq!(arr.iter().missing_integers().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_missing_int_unsigned() {
        let vec = vec![9u8, 8, 8, 5, 4, 3, 2, 0, 2, 4, 5, 8, 10];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![1, 6, 7]);

        let a = usize::MAX as u128;
        let b = a - 1;
        let c = a - 3;
        let d = a + 1;

        let vec = vec![a, b, c];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![a - 2]);

        let vec = vec![a, b, c, d];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![]);

        let vec = vec![8, 8, 8, 8, 8];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![]);
        let vec = vec![4u16, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 8];
        assert_eq!(vec.iter().missing_integers().collect::<Vec<_>>(), vec![]);

        let arr = [0u8; 0];
        assert_eq!(arr.iter().missing_integers().collect::<Vec<_>>(), vec![]);

        let arr = [0u32];
        assert_eq!(arr.iter().missing_integers().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_missing_int_uqsort() {
        let mut vec = vec![];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<i8>>(), vec![]);

        let mut vec = vec![3];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<_>>(), vec![]);

        let mut vec = vec![9i8, -8, -8, -5, -4, -3, -2, 0, 1, 2, 4, 5, 6, 8, 10];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<_>>(), vec![-7, -6, -1, 3, 7]);

        let mut vec = vec![-3, 3];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<_>>(), vec![-2, -1, 0, 1, 2]);

        let mut vec = vec![4u128, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 8];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<_>>(), vec![]);

        let mut vec = vec![9u8, 8, 8, 5, 4, 3, 2, 0, 2, 4, 5, 8, 10];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<_>>(), vec![1, 6, 7]);

        let mut vec = vec![8usize, 8, 8, 8, 8];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<_>>(), vec![]);

        let mut vec = vec![4u16, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 8];
        vec.sort_unstable();
        let iter = vec.iter().unique();
        assert_eq!(iter.missing_integers_uqsort().collect::<Vec<_>>(), vec![]);
    }
}
