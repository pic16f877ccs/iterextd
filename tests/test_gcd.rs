#[cfg(test)]
mod tests {
    use iterextd::IterExtd;

    #[test]
    fn test_gcd() {
        let vec = vec![24, 36, 48];
        assert_eq!(vec.iter().gcd(), Some(12));

        let empty_vec: Vec<i32> = vec![];
        assert_eq!(empty_vec.iter().gcd(), None);

        let single_element = [42];
        assert_eq!(single_element.iter().gcd(), None);

        let no_gcd = vec![2, 3, 5];
        assert_eq!(no_gcd.iter().gcd(), Some(1));
    }
}
