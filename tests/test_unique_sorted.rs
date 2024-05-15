#[cfg(test)]
mod tests {
    use iterextd::IterExtd;

    #[test]
    fn test_unique_sorted_int_signed() {
        let vec = vec![9i8, -8, -8, -5, -4, 4, -3, -2, 0, 1, 8, 2, 4, 10, 5, 6, 8, 10];
        assert_eq!(vec.iter().unique_sorted().collect::<Vec<_>>(), vec![
            -8, -5, -4, -3, -2, 0, 1, 2, 4, 5, 6, 8, 9, 10]);
        assert_eq!([].iter().unique_sorted().collect::<Vec<i16>>(), vec![]);

        assert_eq!([].iter().unique_sorted().rev().collect::<Vec<i16>>(), vec![]);

        assert_eq!(vec.iter().unique_sorted().rev().collect::<Vec<_>>(), vec![
            10, 9, 8, 6, 5, 4, 2, 1, 0, -2, -3, -4, -5, -8]);
        let vec = vec![-4i16, -4, -4, -4, -4, -5, -5, -5, -5, -6, -6, -6, -7, -7, -8];
        assert_eq!(vec.iter().unique_sorted().rev().collect::<Vec<_>>(), vec![-4, -5, -6, -7, -8]);
    }

    #[test]
    fn test_unique_sorted_int_unsigned() {
        let vec = vec![9u8, 8, 5, 4, 3, 2, 0, 1, 8, 2, 4, 10, 5, 6, 8, 10];
        assert_eq!(vec.iter().unique_sorted().collect::<Vec<_>>(), vec![
            0, 1, 2, 3, 4, 5, 6, 8, 9, 10]);

        assert_eq!([].iter().unique_sorted().rev().collect::<Vec<u16>>(), vec![]);

        assert_eq!(vec.iter().unique_sorted().rev().collect::<Vec<_>>(), vec![
            10, 9, 8, 6, 5, 4, 3, 2, 1, 0]);

        let vec = vec![9u8, 8, 5, 4, 3, 2, 2, 2, 8, 2, 4, 10, 5, 6, 8, 10];
        assert_eq!(vec.iter().unique_sorted().collect::<Vec<_>>(), vec![
            2, 3, 4, 5, 6, 8, 9, 10]);

        assert_eq!([].iter().unique_sorted().rev().collect::<Vec<u16>>(), vec![]);

        assert_eq!(vec.iter().unique_sorted().rev().collect::<Vec<_>>(), vec![
            10, 9, 8, 6, 5, 4, 3, 2]);

        let vec = vec![4u16, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 8];
        assert_eq!(vec.iter().unique_sorted().collect::<Vec<_>>(), vec![4, 5, 6, 7, 8]);
    }
}
