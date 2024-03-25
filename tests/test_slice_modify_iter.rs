use iterextd::SliceModifyIter;

#[test]
fn test_gen_range_bounds() {
    let val = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = val.gen_range_bounds(3);
    let vec_cloned = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec_cloned);
    assert_eq!(vec, vec![0..=2, 3..=5, 6..=8, 9..=10]);
    let iter = val.gen_range_bounds(5);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0..=4, 5..=9, 10..=10]);
    let iter = val.gen_range_bounds(25);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0..=10]);
}

#[test]
fn test_gen_range_bounds_len() {
    let val = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut iter = val.gen_range_bounds(3);
    assert_eq!(iter.len(), 4);
    let _ = iter.next();
    assert_eq!((3, Some(3)), iter.size_hint());
    assert_eq!(3, iter.len());
    let iter = val.gen_range_bounds(3).filter(|x| x.start() % 2 == 0);
    assert_eq!((0, Some(4)), iter.size_hint());
    let iter = val
        .gen_range_bounds(3)
        .filter(|x| x.start() % 2 == 0)
        .chain(val.gen_range_bounds(2));
    assert_eq!((6, Some(10)), iter.size_hint());
}

#[test]
fn test_gen_tuple_bounds() {
    let val = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = val.gen_tuple_bounds(3);
    let vec_cloned = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec_cloned);
    assert_eq!(vec, vec![(0, 2), (3, 5), (6, 8), (9, 10)]);
}

#[test]
fn test_gen_tuple_bnds_len() {
    let val = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = val.gen_tuple_bounds(3);
    assert_eq!(iter.len(), 4);
    let mut iter = val.gen_tuple_bounds(3);
    assert_eq!(iter.len(), 4);
    let _ = iter.next();
    assert_eq!((3, Some(3)), iter.size_hint());
    let iter = val.gen_tuple_bounds(3).filter(|x| x.0 % 2 == 0);
    assert_eq!((0, Some(4)), iter.size_hint());
    let iter = val
        .gen_tuple_bounds(3)
        .filter(|x| x.0 % 2 == 0)
        .chain(val.gen_tuple_bounds(2));
    assert_eq!((6, Some(10)), iter.size_hint());
}

#[test]
fn test_slice_modify_iter() {
    let mut arr = [
        "An", "Slice", "modifier", "iterator", "with", "external", "index", "bounds", ".",
    ];
    let size = 5;
    let iter = arr.gen_range_bounds(size);
    let logic = |e: &mut [&str]| {
        let len = e.len();
        if len == size {
            let idx = len - 1;
            let one = e[0];
            e[0] = e[idx];
            e[idx] = one;
        } else {
            let idx = len - 1;
            let one = e[0];
            e[0] = e[idx];
            e[idx] = one;
        }
    };
    let _ = arr.modify_slice(iter, logic);
    assert_eq!(
        arr,
        ["with", "Slice", "modifier", "iterator", "An", ".", "index", "bounds", "external"]
    );

    let mut arr = [
        "An".to_string(),
        "Slice".to_string(),
        "modifier".to_string(),
        "iterator".to_string(),
    ];
    let iter = arr.gen_range_bounds(2).step_by(2);
    let mut idx = arr.gen_range_bounds(2).step_by(2);
    let mut sl = [
        "with".to_string(),
        "external".to_string(),
        "index".to_string(),
        "bounds".to_string(),
        ".".to_string(),
    ];
    let _ = arr.modify_slice(iter, |s| s.swap_with_slice(&mut sl[idx.next().unwrap()]));
    assert_eq!(
        arr,
        [
            "with".to_string(),
            "external".to_string(),
            "modifier".to_string(),
            "iterator".to_string()
        ]
    );
}
