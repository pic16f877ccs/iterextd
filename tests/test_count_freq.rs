#[cfg(test)]
mod tests {
    use iterextd::IterExtd;

    #[test]
    fn test_count_freq() {
        let vec = vec![1, 2, 2, 3, 3, 3];
        let mut count_freq: Vec<_> = vec.into_iter().count_freq().collect();
        count_freq.sort();
        assert_eq!(count_freq, vec![(1, 1), (2, 2), (3, 3)]);

        let s = "Repeated most often";
        let iter = s.chars();
        let mut result = iter.count_freq().collect::<Vec<_>>();
        result.sort();
        assert_eq!(result, vec![(' ', 2), ('R', 1), ('a', 1), ('d', 1), ('e', 4), ('f', 1), ('m', 1), ('n', 1), ('o', 2), ('p', 1), ('s', 1), ('t', 3)]);

        let arr = [1];
        let count_freq = arr.into_iter().count_freq();
        assert_eq!(format!("Count freq: {:?}", count_freq), "Count freq: [(1, 1)]");

        let arr = [1..5];
        let count_freq: Vec<_> = arr.into_iter().count_freq().collect();
        assert_eq!(count_freq, vec![(1..5, 1)]);
    }
}
