use iterextd::IterExtd;

#[test]
fn test_take_skip_cyclic_take_skip_zero() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.iter().take_skip_cyclic(0, 0);
    let vec = iter.collect::<Vec<&u8>>();
    assert_eq!(vec, Vec::<&u8>::new());
}

#[test]
fn test_take_skip_cyclic_take_zero() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.iter().take_skip_cyclic(0, 2);
    let vec = iter.collect::<Vec<&u8>>();
    assert_eq!(vec, Vec::<&u8>::new());
}

#[test]
fn test_take_skip_cyclic_skip_zero() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().take_skip_cyclic(1, 0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_take_skip_cyclic_skip_zero_clone() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().take_skip_cyclic(1, 0);
    let vec_clone = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    let vec_range = (0..=9).collect::<Vec<_>>();
    assert_eq!(vec, vec_range);
    assert_eq!(vec_clone, vec_range);
}

#[test]
fn test_take_skip_cyclic_take_skip_one() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().take_skip_cyclic(1, 1);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_take_skip_cyclic_take_max() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().take_skip_cyclic(20, 3);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_take_skip_cyclic_take_one_skip_max() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().take_skip_cyclic(1, 30);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0,]);
}

#[test]
fn test_take_skip_cyclic_take_nine_skip_one() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().take_skip_cyclic(9, 1);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_take_skip_cyclic_take_nine_skip_max() {
    let arr: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().take_skip_cyclic(9, 10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_take_skip_cyclic_debug() {
    let arr: [u8; 0] = [];
    let iter = arr.iter().take_skip_cyclic(3, 2);
    assert_eq!(
        format!("{:?}", iter),
        "TakeSkipCyclic { iter: Iter([]), take: 3, skip: 2, count: 0 }"
    );
}

#[test]
fn test_take_skip_cyclic_empty() {
    let arr: [u8; 0] = [];
    let iter = arr.iter().take_skip_cyclic(3, 2);
    let vec = iter.collect::<Vec<&u8>>();
    assert_eq!(vec, Vec::<&u8>::new());
}

#[test]
fn test_take_skip_cyclic() {
    let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = arr.iter().take_skip_cyclic(3, 2);
    assert_eq!(iter.collect::<Vec<_>>(), vec![&1, &2, &3, &6, &7, &8]);
}

#[test]
fn test_take_skip_cyclic_rev() {
    let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = arr.iter().take_skip_cyclic(3, 2).rev();
    assert_eq!(iter.collect::<Vec<_>>(), vec![&10, &9, &8, &5, &4, &3]);
}

#[test]
fn test_take_skip_cyclic_chars() {
    let chars = "Iterator adapter".chars();
    let iter = chars.take_skip_cyclic(4, 4);
    assert_eq!(
        iter.collect::<Vec<_>>(),
        vec!['I', 't', 'e', 'r', ' ', 'a', 'd', 'a']
    );
}
