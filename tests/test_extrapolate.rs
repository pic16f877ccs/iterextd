#[cfg(test)]
mod tests {
    use iterextd::IterExtd;

    #[test]
    fn test_extrapolate() {
        let data = vec![2, 5, 6, 9, 13];
        let extrapolated: Vec<_> = data.into_iter().extrapolate().take(10).collect();
        assert_eq!(extrapolated, vec![2, 5, 6, 9, 13, 17, 21, 25, 29, 33]);
        let arr: [u8; 1] = [1];
        let extrapolated: Vec<_> = arr.into_iter().extrapolate().take(5).collect();
        assert_eq!(extrapolated, vec![1, 2, 3, 4, 5]);

        let arr: [i8; 2] = [0, -1];
        let extrapolated: Vec<_> = arr.into_iter().extrapolate().take(5).collect();
        assert_eq!(extrapolated, vec![0, -1, -2, -3, -4]);

        let arr: [f32; 2] = [0.1, 0.2];
        let iter = arr.into_iter().extrapolate();
        let iter_clone = iter.clone();
        let extrapolated: Vec<_> = iter_clone.take(5).collect();
        let extrapolated_clone: Vec<_> = iter.take(5).collect();
        assert_eq!(extrapolated, extrapolated_clone);
        let arr: [u8; 4] = [2, 4, 6, 8];
        let iter = arr.into_iter().extrapolate();
        assert_eq!(format!("{:?}", iter), "Extrapolate { iter: IntoIter([2, 4, 6, 8]), arg_one: 0, arg_two: 0 }");
    }
}
