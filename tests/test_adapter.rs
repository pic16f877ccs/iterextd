use iterextd::IterExtd;

#[test]
fn test_adapter() {
    let arr: [u8; 3] = [40, 50, 60];
    let iter = arr.iter().adapter([10, 20, 30].iter(), |iter, i| {
        if let Some(result) = i.next() {
            Some(result)
        } else {
            iter.next()
        }
    });
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![&10, &20, &30, &40, &50, &60]);
}

#[test]
fn test_adapter_empty() {
    let arr: [u8; 0] = [];
    let iter = arr.into_iter().adapter(0, |iter, _x| iter.next());
    let vec = iter.collect::<Vec<u8>>();
    assert_eq!(vec, Vec::<u8>::new());
}

#[test]
fn test_adapter_debug() {
    let mut arr_first = [10u8, 11, 12];
    let mut arr_second = [13, 14];
    let first_iter = arr_first.iter_mut();
    let second_iter = arr_second.iter_mut();
    let iter = first_iter.adapter(second_iter, |iter_first, iter_second| {
        Some([iter_first.next()?, iter_second.next()?])
    });
    assert_eq!(
        format!("{:?}", iter),
        "Adapter { iter_self: IterMut([10, 11, 12]), other: IterMut([13, 14]) }"
    );
}

#[test]
fn test_adapter_mut() {
    let mut arr_first = [10u8, 11, 12];
    let mut arr_second = [13, 14];
    let first_iter = arr_first.iter_mut();
    let second_iter = arr_second.iter_mut();
    let iter = first_iter.adapter(second_iter, |iter_first, iter_second| {
        Some([iter_first.next()?, iter_second.next()?])
    });
    assert_eq!(
        iter.collect::<Vec<_>>(),
        vec![[&mut 10, &mut 13], [&mut 11, &mut 14]]
    );

}

#[test]
fn test_adapter_clone() {
    let arr: [u8; 3] = [4, 5, 6];
    let iter = arr.into_iter().adapter([1, 2, 3].iter(), |iter, i| {
        if let Some(result) = i.next() {
            Some(*result)
        } else {
            iter.next()
        }
    });
    let vec_cloned = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec_cloned);
}
