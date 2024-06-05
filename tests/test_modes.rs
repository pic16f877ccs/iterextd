#[cfg(test)]
mod tests {
    use iterextd::IterExtd;

    #[test]
    fn test_modes() {
        let vec = vec![1, 2, 2, 3, 3, 3];
        let modes: Vec<_> = vec.into_iter().modes().collect();
        assert_eq!(modes, vec![(3, 3)]);

        let vec = vec![1..5, 2..5, 2..2, 2..5, 3..9, 3..9, 3..9];
        let modes: Vec<_> = vec.into_iter().modes().collect();
        assert_eq!(modes, vec![(3..9, 3)]);

        let vec = vec!['a', 'b', 'b', 'c', 'c', 'c'];
        let modes: Vec<_> = vec.into_iter().modes().collect();
        assert_eq!(modes, vec![('c', 3)]);

        let vec = vec!["10", "20", "20", "30", "30", "30"];
        let modes: Vec<_> = vec.iter().modes().collect();
        assert_eq!(modes, vec![(&"30", 3)]);

        let data = "10, 20, 20, 30, 30, 30";
        let modes: Vec<_> = data.chars().modes().collect();
        assert_eq!(modes, vec![('0', 6)]);

        let arr = [1];
        let mode = arr.into_iter().modes();
        assert_eq!(format!("Mode: {:?}", mode), "Mode: Filter { iter: [(1, 1)] }");
    }
}
