use iterextd::TupleIntoIter;

#[test]
fn test_tuple_into_iter() {
    let tup = (1, 2, 3, 4, 5, 6, 7, 8);
    let vec = tup.tuple_into_iter().collect::<Vec<_>>();
    assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8,]);
    println!("{:?}", tup);
}

#[test]
fn test_tuple_into_iter_vec() {
    let tup = (vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]);
    let vec = tup.tuple_into_iter().collect::<Vec<_>>();
    assert_eq!(vec, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]]);
}

#[test]
fn test_tuple_into_iter_string() {
    let tup = ("tuple".to_string(), "into".to_string(), "iter".to_string());
    let vec = tup.tuple_into_iter().collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec!["tuple".to_string(), "into".to_string(), "iter".to_string()]
    );
}

#[test]
fn test_tuple_into_iter_12() {
    let tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    let mut iter = tup.tuple_into_iter();
    for i in 1..13 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_11() {
    let tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    let mut iter = tup.tuple_into_iter();
    for i in 1..12 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_10() {
    let tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let mut iter = tup.tuple_into_iter();
    for i in 1..11 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_9() {
    let tup = (1, 2, 3, 4, 5, 6, 7, 8, 9);
    let mut iter = tup.tuple_into_iter();
    for i in 1..10 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_8() {
    let tup = (1, 2, 3, 4, 5, 6, 7, 8);
    let mut iter = tup.tuple_into_iter();
    for i in 1..9 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_7() {
    let tup = (1, 2, 3, 4, 5, 6, 7);
    let mut iter = tup.tuple_into_iter();
    for i in 1..8 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_6() {
    let tup = (1, 2, 3, 4, 5, 6);
    let mut iter = tup.tuple_into_iter();
    for i in 1..7 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_5() {
    let tup = (1, 2, 3, 4, 5);
    let mut iter = tup.tuple_into_iter();
    for i in 1..6 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_4() {
    let tup = (1, 2, 3, 4);
    let mut iter = tup.tuple_into_iter();
    for i in 1..5 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_3() {
    let tup = (1, 2, 3);
    let mut iter = tup.tuple_into_iter();
    for i in 1..4 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_2() {
    let tup = (1, 2);
    let mut iter = tup.tuple_into_iter();
    for i in 1..3 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_into_iter_1() {
    let tup = (1,);
    let mut iter = tup.tuple_into_iter();
    for i in 1..2 {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}
