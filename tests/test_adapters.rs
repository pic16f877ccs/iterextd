use core::iter;
use iterextd::IterExtd;
use trybuild::TestCases;

#[test]
fn test_collect_zeroed_arr() {
    let arr = [('a', 20u8), ('b', 22), ('c', 122)];
    let iter = arr.iter().map(|(a, b)| (*a, *b));
    let zeroed_arr: (usize, [_; 5]) = iter.collect_arr_zeroed();
    assert_eq!(
        zeroed_arr,
        (
            3,
            [('a', 20u8), ('b', 22), ('c', 122), ('\0', 0), ('\0', 0)]
        )
    );

    let arr = [
        &[10u8, 11, 22],
        &[33, 44, 55],
        &[66, 77, 88],
        &[99, 111, 222],
    ];
    let iter = arr.iter().copied().arr_chunks::<3>().array_copied();
    let val: (usize, [[[u8; 3]; 3]; 2]) = iter.collect_arr_zeroed();
    assert_eq!(
        val,
        (
            1,
            [
                [[10u8, 11, 22], [33, 44, 55], [66, 77, 88]],
                [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
            ]
        )
    );

    let arr = [
        &[10u8, 11, 22],
        &[33, 44, 55],
        &[66, 77, 88],
        &[99, 111, 222],
    ];
    let iter = arr.iter().copied().arr_chunks::<4>().array_copied();
    let val: (usize, [[[u8; 3]; 4]; 1]) = iter.collect_arr_zeroed();
    assert_eq!(
        val,
        (
            1,
            [[[10u8, 11, 22], [33, 44, 55], [66, 77, 88], [99, 111, 222]]]
        )
    );
}

#[test]
#[ignore = "arr_zeroed_zero"]
fn test_collect_zeroed_arr_non_zero() {
    let test_ui = TestCases::new();
    test_ui.compile_fail("tests/ui/test_ui_arr_zeroed_non_zero.rs");
}

#[test]
#[ignore = "arr_zeroed_zts"]
fn test_collect_zeroed_arr_zts() {
    let test_ui = TestCases::new();
    test_ui.compile_fail("tests/ui/test_ui_arr_zeroed_zts.rs");
}

#[test]
#[ignore = "arr_zeroed_ref"]
fn test_collect_zeroed_arr_ref() {
    let test_ui = TestCases::new();
    test_ui.compile_fail("tests/ui/test_ui_arr_zeroed_ref.rs");
}

#[test]
#[should_panic]
#[ignore = "arr_zeroed_const"]
fn test_collect_zeroed_arr_const_zero() {
    let data = vec![1010, 2020, 3030].into_iter();
    let (_index, _arr) = data.collect_arr_zeroed::<0>();
}

#[test]
fn test_array_chunks() {
    let arr = [&1u16, &2, &3, &4, &5, &6, &7, &8, &9, &10];
    let iter = arr.iter();
    let vec = iter.arr_chunks().collect::<Vec<[&&u16; 5]>>();
    assert_eq!(
        vec,
        vec![[&&1u16, &&2, &&3, &&4, &&5], [&&6, &&7, &&8, &&9, &&10]]
    );

    let arr = [1u16, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = arr.iter();
    let vec = iter.clone().arr_chunks().collect::<Vec<[&u16; 5]>>();
    assert_eq!(vec, vec![[&1u16, &2, &3, &4, &5], [&6, &7, &8, &9, &10]]);

    let vec = iter.arr_chunks().collect::<Vec<[&u16; 8]>>();
    assert_eq!(vec, vec![[&1u16, &2, &3, &4, &5, &6, &7, &8]]);

    let arr = [];
    let iter = arr.iter();
    let vec = iter.arr_chunks().collect::<Vec<[&u16; 1]>>();
    assert_eq!(vec, Vec::<[&u16; 1]>::new());

    let vec = vec!["one", "two", "three"];
    let iter = vec.iter().arr_chunks::<2>();
    let iter_cloned = iter.clone();
    let vec_left = iter.collect::<Vec<_>>();
    let vec_right = iter_cloned.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);

    let vec = vec!["one"];
    let iter = vec.iter().arr_chunks::<1>();
    assert_eq!(
        format!("{:?}", iter),
        "ArrChunks { iter: Iter([\"one\"]) }".to_string()
    );
}

#[test]
#[should_panic]
#[ignore = "arr_chunks_const"]
fn test_arr_chunks_const_zero() {
    let data = vec![10.10_f32, 20.20, 30.30].into_iter();
    let _ = data.arr_chunks::<0>();
}

#[test]
fn test_arr_chunks_infer() {
    let xs = [1, 1, 2, -2, 6, 0, 3, 1];
    for [a, b, c] in xs.iter().copied().arr_chunks() {
        assert_eq!(a + b + c, 4);
    }
}

#[test]
fn test_arr_chunks_count() {
    let it = (0..6).arr_chunks::<1>();
    assert_eq!(it.count(), 6);

    let it = (0..6).arr_chunks::<3>();
    assert_eq!(it.count(), 2);

    let it = (0..6).arr_chunks::<5>();
    assert_eq!(it.count(), 1);

    let it = (0..6).arr_chunks::<7>();
    assert_eq!(it.count(), 0);

    let it = (0..6).filter(|x| x % 2 == 0).arr_chunks::<2>();
    assert_eq!(it.count(), 1);

    let it = iter::empty::<i32>().arr_chunks::<2>();
    assert_eq!(it.count(), 0);

    let it = [(); usize::MAX].iter().arr_chunks::<2>();
    assert_eq!(it.count(), usize::MAX / 2);
}

#[test]
fn test_arr_chunks_size_hint() {
    let it = (0..6).arr_chunks::<1>();
    assert_eq!(it.size_hint(), (6, Some(6)));

    let it = (0..6).arr_chunks::<3>();
    assert_eq!(it.size_hint(), (2, Some(2)));

    let it = (0..6).arr_chunks::<5>();
    assert_eq!(it.size_hint(), (1, Some(1)));

    let it = (0..6).arr_chunks::<7>();
    assert_eq!(it.size_hint(), (0, Some(0)));

    let it = (1..).arr_chunks::<2>();
    assert_eq!(it.size_hint(), (usize::MAX / 2, None));

    let it = (1..).filter(|x| x % 2 != 0).arr_chunks::<2>();
    assert_eq!(it.size_hint(), (0, None));
}

#[test]
fn test_collect_array() {
    let arr = [('a', 20u8), ('b', 22), ('c', 122)];
    let iter = arr.iter();
    let coll_arr: [_; 3] = iter.copied().collect_array();
    assert_eq!(coll_arr, arr);

    let arr = [[10u8, 11, 22], [33, 44, 55], [66, 77, 88], [99, 111, 222]];
    let iter = arr.iter().arr_chunks::<2>();
    let val: [[&[u8; 3]; 2]; 2] = iter.collect_array();
    assert_eq!(
        val,
        [
            [&[10u8, 11, 22], &[33, 44, 55]],
            [&[66, 77, 88], &[99, 111, 222]]
        ]
    );

    let arr = [
        &[10u8, 11, 22],
        &[33, 44, 55],
        &[66, 77, 88],
        &[99, 111, 222],
    ];
    let iter = arr.iter();
    let val: [&&[u8; 3]; 4] = iter.collect_array();
    assert_eq!(
        val,
        [
            &&[10u8, 11, 22],
            &&[33, 44, 55],
            &&[66, 77, 88],
            &&[99, 111, 222]
        ]
    );

    let arr = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut iter = arr.iter();
    let iter_by_ref = iter.by_ref();
    let coll_arr: [_; 3] = iter_by_ref.collect_array();
    assert_eq!(coll_arr, [&1u8, &2, &3]);

    let arr = vec![vec![1u8, 2], vec![9, 10]];
    let iter = arr.iter();
    let coll_arr: [_; 2] = iter.collect_array();
    assert_eq!(coll_arr, [&vec![1u8, 2], &vec![9, 10]]);
}

#[test]
#[should_panic]
#[ignore = "arr_collect_const"]
fn test_collect_array_const_zero() {
    let data = vec![100, 200, 300].into_iter();
    let _arr = data.collect_array::<0>();
}

#[test]
#[should_panic]
#[ignore = "arr_collect_const"]
fn test_collect_array_uninit() {
    let data = vec![100, 200, 300].into_iter();
    let _arr = data.collect_array::<4>();
}

#[test]
fn test_comb_two_iters() {
    let basic_list = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let other_list = [10u8, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let basic_iter = basic_list.iter();
    let other_iter = other_list.iter();

    let combined = basic_iter.clone().combine_iters(1, other_iter.clone(), 3);
    let vec = combined.cloned().collect::<Vec<_>>();
    assert_eq!(vec, [0u8, 10, 11, 12, 1, 13, 14, 15, 2, 16, 17, 18, 3, 19]);

    let combined = basic_iter.clone().combine_iters(1, other_iter.clone(), 0);
    let vec = combined.cloned().collect::<Vec<_>>();
    assert_eq!(vec, [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let combined = basic_iter.clone().combine_iters(10, other_iter.clone(), 0);
    let vec = combined.cloned().collect::<Vec<_>>();
    assert_eq!(vec, [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let combined = basic_iter.clone().combine_iters(11, other_iter.clone(), 0);
    let vec = combined.cloned().collect::<Vec<_>>();
    assert_eq!(vec, [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let vec_self = vec!["one", "two", "three"];
    let vec_other = vec!["first", "second", "third", "fourth"];
    let iter_other = vec_other.iter();
    let iter = vec_self.iter().combine_iters(2, iter_other, 2);
    let iter_cloned = iter.clone();
    let vec_left = iter.collect::<Vec<_>>();
    let vec_right = iter_cloned.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);

    let len = 4;
    let vec_self = vec!["one", "two", "three"];
    let iter_self = vec_self.iter();
    let vec_other = vec!["first", "second", "third", "fourth"];
    let iter_other = vec_other.iter();
    let vec_left = iter_self
        .combine_iters(0, iter_other.clone(), len)
        .collect::<Vec<_>>();
    let vec_right = iter_other.collect::<Vec<_>>();
    assert_eq!(vec_left.len(), len);
    assert_eq!(vec_left, vec_right);

    let len = 5;
    let vec_self = vec!["one", "two", "three"];
    let iter_self = vec_self.iter();
    let vec_other = vec!["first", "second", "third", "fourth"];
    let iter_other = vec_other.iter();
    let vec_left = iter_self
        .combine_iters(0, iter_other.clone(), len)
        .collect::<Vec<_>>();
    let vec_right = iter_other.collect::<Vec<_>>();
    assert_ne!(vec_left.len(), len);
    assert_eq!(vec_left, vec_right);

    let vec_self = vec!["one", "two", "three"];
    let iter_self = vec_self.iter();
    let vec_other = vec!["first", "second", "third", "fourth"];
    let iter_other = vec_other.iter();
    let vec_left = iter_self
        .clone()
        .combine_iters(4, iter_other, 1)
        .collect::<Vec<_>>();
    let vec_right = iter_self.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);

    let vec_one = vec!["one"];
    let vec_two = vec!["two"];
    let iter = vec_one.iter().combine_iters(1, vec_two.iter(), 1);
    assert_eq!(format!("{:?}", iter),
    "CombineIters { self_iter: Iter([\"one\"]), self_part_len: 1, self_counter: 0, other_iter: Iter([\"two\"]), other_part_len: 1, other_counter: 1 }".to_string());

    let vec_one: Vec<u8> = vec![];
    let vec_two: Vec<u8> = vec![];
    let iter = vec_one.iter().combine_iters(1, vec_two.iter(), 1);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, Vec::<&u8>::new());
}

#[test]
#[should_panic]
#[ignore = "comb_two_iters_two_val_zero"]
fn test_comb_two_iters_values_zero() {
    let basic_list = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let other_list = [10u8, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let basic_iter = basic_list.iter();
    let other_iter = other_list.iter();
    let _combined = basic_iter.combine_iters(0, other_iter.clone(), 0);
}

#[test]
fn test_copied() {
    let vec = vec![0, 10, 27, 38, 49, 50];
    let iter = vec.iter().arr_chunks().array_copied();
    let vec_copied = iter.collect::<Vec<_>>();
    assert_eq!(vec_copied, vec![[0, 10], [27, 38], [49, 50]]);

    let vec = vec![&0usize, &10, &27, &38, &49, &50];
    let iter = vec.iter().arr_chunks().array_copied().array_copied();
    let vec_copied = iter.collect::<Vec<_>>();
    assert_eq!(vec_copied, vec![[0usize, 10, 27], [38, 49, 50]]);

    let vec = vec!["usize", "isize", "float", "typle", "struct", "enum"];
    let iter = vec.iter().arr_chunks().array_copied();
    let vec_copied = iter.collect::<Vec<_>>();
    assert_eq!(
        vec_copied,
        vec![["usize", "isize"], ["float", "typle"], ["struct", "enum"]]
    );

    let vec: Vec<()> = vec![(), (), (), ()];
    let iter = vec.iter().arr_chunks().array_copied();
    let vec_copied = iter.collect::<Vec<_>>();
    assert_eq!(vec_copied, vec![[(), (), (), ()]]);

    let vec: Vec<char> = vec!['c', 'o', 'p', 'i', 'e', 'd'];
    let iter = vec.iter().arr_chunks().array_copied();
    let vec_copied = iter.collect::<Vec<_>>();
    assert_eq!(vec_copied, vec![['c', 'o', 'p'], ['i', 'e', 'd']]);

    let vec: Vec<char> = vec!['c'];
    let iter = vec.iter().arr_chunks::<1>().array_copied();
    assert_eq!(
        &format!("{:?}", iter),
        "ArrayCopied { iter: ArrChunks { iter: Iter(['c']) } }"
    );

    let vec: Vec<char> = vec!['c', 'o', 'p', 'i', 'e', 'd'];
    let iter_left = vec.iter().arr_chunks::<3>().array_copied();
    let iter_right = iter_left.clone();

    let iter_copied_left = iter_left.collect::<Vec<_>>();
    let iter_copied_right = iter_right.collect::<Vec<_>>();
    assert_eq!(iter_copied_left, iter_copied_right);
}

#[test]
fn test_cloned() {
    let vec = vec![0, 10, 27, 38, 49, 50];
    let iter = vec.iter().arr_chunks().array_cloned();
    let vec_cloned = iter.collect::<Vec<_>>();
    assert_eq!(vec_cloned, vec![[0, 10], [27, 38], [49, 50]]);

    let vec = vec![&0usize, &10, &27, &38, &49, &50];
    let iter = vec.iter().arr_chunks().array_cloned().array_cloned();
    let vec_cloned = iter.collect::<Vec<_>>();
    assert_eq!(vec_cloned, vec![[0usize, 10, 27], [38, 49, 50]]);

    let vec = vec!["usize", "isize", "float", "typle", "struct", "enum"];
    let iter = vec.iter().arr_chunks().array_cloned();
    let vec_cloned = iter.collect::<Vec<_>>();
    assert_eq!(
        vec_cloned,
        vec![["usize", "isize"], ["float", "typle"], ["struct", "enum"]]
    );

    let vec: Vec<()> = vec![(), (), (), ()];
    let iter = vec.iter().arr_chunks().array_cloned();
    let vec_cloned = iter.collect::<Vec<_>>();
    assert_eq!(vec_cloned, vec![[(), (), (), ()]]);

    let vec: Vec<char> = vec!['c', 'l', 'o', 'n', 'e', 'd'];
    let iter = vec.iter().arr_chunks().array_cloned();
    let vec_cloned = iter.collect::<Vec<_>>();
    assert_eq!(vec_cloned, vec![['c', 'l', 'o'], ['n', 'e', 'd']]);

    let vec = vec![vec!["usize"], vec!["isize"], vec!["float"], vec!["typle"]];
    let iter = vec.iter().arr_chunks().array_cloned();
    let vec_cloned = iter.collect::<Vec<_>>();
    assert_eq!(
        vec_cloned,
        vec![
            [vec!["usize"], vec!["isize"]],
            [vec!["float"], vec!["typle"]]
        ]
    );

    let vec: Vec<char> = vec!['c'];
    let iter = vec.iter().arr_chunks::<1>().array_cloned();
    assert_eq!(
        &format!("{:?}", iter),
        "ArrayCloned { iter: ArrChunks { iter: Iter(['c']) } }"
    );

    let vec = vec![vec!["usize"], vec!["isize"], vec!["float"], vec!["typle"]];
    let iter_left = vec.iter().arr_chunks::<2>().array_cloned();
    let iter_right = iter_left.clone();
    let vec_cloned_left = iter_left.collect::<Vec<_>>();
    let vec_cloned_right = iter_right.collect::<Vec<_>>();
    assert_eq!(vec_cloned_left, vec_cloned_right);
}

#[test]
fn test_skip_step_by() {
    let vec = vec![10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99];
    let iter = vec.iter();

    let iter_cloned = iter.clone().skip_step_by(2, 3);
    let vec = iter_cloned.cloned().collect::<Vec<_>>();
    assert_eq!(vec, vec![22, 55, 88]);

    let iter_cloned = iter.clone().skip_step_by(0, 3);
    let vec = iter_cloned.cloned().collect::<Vec<_>>();
    assert_eq!(vec, vec![10, 33, 66, 99]);

    let vec_l = (0..11).step_by(1).collect::<Vec<_>>();
    let vec_r = (0..11).skip_step_by(0, 1).collect::<Vec<_>>();
    assert_eq!(vec_l, vec_r);

    let vec_l = (0..11).step_by(2).collect::<Vec<_>>();
    let vec_r = (0..11).skip_step_by(0, 2).collect::<Vec<_>>();
    assert_eq!(vec_l, vec_r);

    let vec_l = (0..11).step_by(3).collect::<Vec<_>>();
    let vec_r = (0..11).skip_step_by(0, 3).collect::<Vec<_>>();
    assert_eq!(vec_l, vec_r);

    let vec: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let iter = vec.iter().skip_step_by(1, 2);
    assert_eq!(
        &format!("{:?}", iter),
        "SkipStepBy { iter: Fuse { iter: Some(Iter(['a', 'b', 'c', 'd'])) }, skip: 1, step: 1 }"
    );

    let vec: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let iter = vec.iter().skip_step_by(1, 2);
    let vec_right = iter.clone().collect::<Vec<_>>();
    let vec_left = iter.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);
}

#[test]
fn test_skip_step_size_hint() {
    let iter = (0..6).skip_step_by(0, 1);
    assert_eq!(iter.size_hint(), (6, Some(6)));

    let iter = (0..6).skip_step_by(0, 3);
    assert_eq!(iter.size_hint(), (2, Some(2)));

    let iter = (0..6).skip_step_by(0, 6);
    assert_eq!(iter.size_hint(), (1, Some(1)));

    let iter = (0..6).skip_step_by(7, 1);
    assert_eq!(iter.size_hint(), (0, Some(0)));

    let iter = iter::empty::<i32>().skip_step_by(1, 1);
    assert_eq!(iter.size_hint(), (0, Some(0)));
}

#[test]
fn test_skip_step_count() {
    let iter_left = (0..10).skip_step_by(5, 1);
    let iter_right = (0..10).skip(5).step_by(1);
    assert_eq!(iter_left.count(), iter_right.count());

    let iter_left = (0..6).skip_step_by(0, 3);
    let iter_right = (0..6).step_by(3);
    assert_eq!(iter_left.count(), iter_right.count());

    let iter_left = (0..6).skip_step_by(0, 5);
    let iter_right = (0..6).step_by(5);
    assert_eq!(iter_left.count(), iter_right.count());

    let iter = (0..6).skip_step_by(0, 7);
    assert_eq!(iter.count(), 1);

    let iter = (0..6).skip_step_by(7, 1);
    assert_eq!(iter.count(), 0);

    let iter_left = (0..6).filter(|x| x % 2 == 0).skip_step_by(0, 2);
    let iter_right = (0..6).filter(|x| x % 2 == 0).skip(0).step_by(2);
    assert_eq!(iter_left.count(), iter_right.count());

    let iter = iter::empty::<i32>().skip_step_by(2, 2);
    assert_eq!(iter.count(), 0);
}

#[test]
#[should_panic]
#[ignore = "skip_step_zero"]
fn test_skip_step_by_step_zero() {
    let arr = [10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99];
    let _iter = arr.iter().skip_step_by(5, 0);
}

#[test]
fn test_step_by_fn() {
    let arr = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.iter();
    let iter_cloned = iter.clone().step_by_fn(|s| {
        if *s == 0 {
            *s = 1;
            1
        } else {
            *s += 1;
            *s
        }
    });
    let vec = iter_cloned.collect::<Vec<_>>();
    assert_eq!(vec, [&0, &2, &5, &9]);

    let vec = iter
        .step_by_fn(|s| {
            *s = 0;
            *s
        })
        .collect::<Vec<_>>();
    assert_eq!(vec, Vec::<&u8>::new());

    let vec = vec![0u32, 11, 22, 33, 44, 55, 66, 77, 88, 99];
    let iter = vec.into_iter().step_by_fn(|s| {
        if *s == 0 {
            *s = 1;
            1
        } else {
            *s += 2;
            *s
        }
    });
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, [0, 33, 88]);

    let mut arr = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.iter_mut().step_by_fn(|s| {
        if *s == 0 {
            *s = 1;
            1
        } else {
            *s += 1;
            *s
        }
    });
    iter.for_each(|elem| *elem *= 10);
    assert_eq!(arr, [0u8, 1, 20, 3, 4, 50, 6, 7, 8, 90]);

    let iter = (0..5).step_by_fn(|s| {
        *s = 1;
        *s
    });
    let vec_left = iter.collect::<Vec<_>>();
    let iter = (0..5).step_by(1);
    let vec_right = iter.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);

    let iter = (0..9).step_by_fn(|s| {
        *s = 2;
        *s
    });
    let vec_left = iter.collect::<Vec<_>>();
    let iter = (0..9).skip(1).step_by(2);
    let vec_right = iter.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);

    let iter = (0..9).step_by_fn(|s| {
        if *s == 0 {
            *s = 1;
            1
        } else {
            *s = 2;
            *s
        }
    });

    let vec_left = iter.collect::<Vec<_>>();
    let iter = (0..9).step_by(2);
    let vec_right = iter.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);

    let vec: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let iter = vec.iter().step_by_fn(|s| {
        if *s == 0 {
            *s = 1;
            1
        } else {
            *s = 2;
            *s
        }
    });

    let vec_right = iter.clone().collect::<Vec<_>>();
    let vec_left = iter.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);
}

#[test]
fn test_slice_copied() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7];
    let arr_of_slices = [&arr[0..2], &arr[2..4], &arr[4..6], &arr[6..8]];
    let iter = arr_of_slices.into_iter().slice_copied::<2>();
    let vec = iter.collect::<Vec<[u8; 2]>>();
    assert_eq!(vec, vec![[0, 1], [2, 3], [4, 5], [6, 7]]);

    let vec = {
        let arr = [10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222];
        let arr_sl: [&[u8]; 3] = [&arr[0..3], &arr[3..6], &arr[6..9]];
        let iter = arr_sl.iter().copied().slice_copied::<3>();
        iter.collect::<Vec<_>>()
    };
    assert_eq!(vec, vec![[10, 11, 22], [33, 44, 55], [66, 77, 88]]);

    let arr = [0, 1, 2, 3, 4, 5, 6, 7];
    let arr_of_slices = [&arr[0..2], &arr[2..4], &arr[4..6]];
    let iter = arr_of_slices.into_iter().slice_copied::<2>();
    assert_eq!(
        format!("{:?}", iter),
        "SliceCopied { iter: IntoIter([[0, 1], [2, 3], [4, 5]]) }".to_string()
    );

    let arr = ["one", "two", "three", "four"];
    let arr_of_slices = [&arr[0..2], &arr[2..4]];
    let left_iter = arr_of_slices.into_iter().slice_copied::<2>();
    let right_iter = left_iter.clone();
    let left_vec = left_iter.collect::<Vec<_>>();
    let right_vec = right_iter.collect::<Vec<_>>();
    assert_eq!(left_vec, right_vec);
}

#[test]
#[should_panic]
#[ignore = "slice_copied_eq_lens"]
fn test_slice_copied_eq_lens() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let arr_of_slices = [&arr[0..2], &arr[2..4], &arr[4..6], &arr[6..9]];
    let iter = arr_of_slices.into_iter().slice_copied::<2>();
    let vec = iter.collect::<Vec<[u8; 2]>>();
    assert_eq!(vec, vec![[0, 1], [2, 3], [4, 5], [6, 7]]);
}

#[test]
fn test_last_item() {
    let arr = [10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222];
    let mut iter = arr.into_iter().last_taken();
    assert_eq!(iter.last_item(), None);
    let _vec = iter.by_ref().take(2).collect::<Vec<_>>();
    assert_eq!(Some(11), iter.last_item().copied());
    let _vec = iter.by_ref().collect::<Vec<_>>();
    assert_eq!(Some(222), iter.last_item().copied());
}

#[test]
fn test_map_iter() {
    let months = vec![
        "december", "january", "february", "march", "april", "may", "june", "july", "august",
    ];
    let seasons = vec!["winter", "spring", "summer"];
    let seasons_iter = seasons.into_iter();
    let iter = months.into_iter().map_iters(seasons_iter, |m, s| {
        Some((m.next()?, m.next()?, m.next()?, s.next()?))
    });
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![
            ("december", "january", "february", "winter"),
            ("march", "april", "may", "spring"),
            ("june", "july", "august", "summer")
        ]
    );

    let a_arr = [10, 20, 30, 40, 50];
    let b_arr = [];
    let a_iter = a_arr.iter().copied();
    let b_iter = b_arr.iter().copied();
    let iter = a_iter.map_iters(b_iter, |a, b| Some([a.next()?, b.next()?]));
    let vec = iter.collect::<Vec<[i32; 2]>>();
    assert_eq!(vec, Vec::<[i32; 2]>::from([]));

    let a_iter = b_arr.iter().copied();
    let b_iter = a_arr.iter().copied();
    let iter = a_iter.map_iters(b_iter, |a, b| Some([a.next()?, b.next()?]));
    let vec = iter.collect::<Vec<[i32; 2]>>();
    assert_eq!(vec, Vec::<[i32; 2]>::from([]));

    let b_arr = [1];
    let a_iter = a_arr.iter().copied();
    let b_iter = b_arr.iter().copied();
    let iter = a_iter.map_iters(b_iter, |a, b| Some([a.next()?, b.next()?]));
    let vec = iter.collect::<Vec<[i32; 2]>>();
    assert_eq!(vec, Vec::<[i32; 2]>::from([[10, 1]]));

    let a_iter = b_arr.iter().copied();
    let b_iter = a_arr.iter().copied();
    let iter = a_iter.map_iters(b_iter, |a, b| Some([a.next()?, b.next()?]));
    let vec = iter.collect::<Vec<[i32; 2]>>();
    assert_eq!(vec, Vec::<[i32; 2]>::from([[1, 10]]));

    let b_arr = [1, 2, 3];
    let a_iter = a_arr.iter();
    let b_iter = b_arr.iter();
    let iter = a_iter.map_iters(b_iter, |a, b| Some([a.next()?, b.next()?]));

    let vec = iter.collect::<Vec<[&i32; 2]>>();
    assert_eq!(
        vec,
        Vec::<[&i32; 2]>::from([[&10, &1], [&20, &2], [&30, &3]])
    );
}

#[test]
fn test_map_by_two() {
    let arr = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let iter = arr.into_iter().map_by_two(|a, b| (b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![(2, 1), (4, 3), (6, 5), (8, 7)]);

    let iter = vec.iter().map_by_two(|a, b| [a.0, a.1, b.0, b.1]).flatten();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![2, 1, 4, 3, 6, 5, 8, 7]);

    let vec = vec!["map".to_string(), "by".to_string(), "two".to_string()];
    let iter = vec.into_iter().map_by_two(|a, b| (b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![("by".to_string(), "map".to_string())]);

    let vec = vec!["map".to_string(), "by".to_string(), "two".to_string()];
    let iter = vec.iter().map_by_two(|a, b| (b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![(&"by".to_string(), &"map".to_string())]);

    let arr: Vec<u8> = vec![];
    let iter = arr.into_iter().map_by_two(|a, b| (b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);

    let arr = [1u8];
    let iter = arr.into_iter().map_by_two(|a, b| (b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);

    let vec = vec!["one", "two", "three"];
    let iter = vec.into_iter().map_by_two(|a, b| (b, a));
    let vec_left = iter.collect::<Vec<_>>();
    assert_eq!(vec_left, vec![("two", "one")]);

    let vec = vec!["one"];
    let iter = vec.into_iter().map_by_two(|a, b| (b, a));
    let vec_left = iter.collect::<Vec<_>>();
    assert_eq!(vec_left, vec![]);

    let vec = vec!["one", "two", "three"];
    let iter = vec.iter().map_by_two(|a, b| (b, a));
    let iter_cloned = iter.clone();
    let vec_left = iter.collect::<Vec<_>>();
    let vec_right = iter_cloned.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);
}

#[test]
fn test_map_by_three() {
    let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = arr.into_iter().map_by_three(|a, b, c| (c, b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![(3, 2, 1), (6, 5, 4), (9, 8, 7)]);

    let vec = vec!["map".to_string(), "by".to_string(), "three".to_string()];
    let iter = vec.into_iter().map_by_three(|a, b, c| (c, b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![("three".to_string(), "by".to_string(), "map".to_string())]
    );

    let vec = vec!["map".to_string(), "by".to_string(), "three".to_string()];
    let iter = vec.iter().map_by_three(|a, b, c| (c, b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![(&"three".to_string(), &"by".to_string(), &"map".to_string())]
    );

    let arr: Vec<u8> = vec![];
    let iter = arr.into_iter().map_by_three(|a, b, c| (b, a, c));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);

    let arr = [1u8];
    let iter = arr.into_iter().map_by_three(|a, b, c| (c, b, a));
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);

    let vec = vec!["one", "two", "three"];
    let iter = vec.iter().map_by_three(|a, b, c| (c, b, a));
    let iter_cloned = iter.clone();
    let vec_left = iter.collect::<Vec<_>>();
    let vec_right = iter_cloned.collect::<Vec<_>>();
    assert_eq!(vec_left, vec_right);
}

#[test]
fn test_consume() {
    let mut arr = [1, 2, 3];
    let _ = arr
        .iter_mut()
        .map(|elem| {
            *elem *= 10;
        })
        .consume();
    assert_eq!(arr, [10, 20, 30]);
}

#[test]
fn test_inclusive_step_by() {
    let arr: [u8; 0] = [];

    let mut step = 1;
    let vec_ext = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    let vec_std = arr.into_iter().step_by(step).collect::<Vec<_>>();
    assert_eq!(vec_ext, vec_std);

    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    let vec_ext = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    let vec_std = arr.into_iter().step_by(step).collect::<Vec<_>>();
    assert_eq!(vec_ext, vec_std);

    step = 2;
    let vec_ext = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    let vec_std = arr.into_iter().step_by(step).collect::<Vec<_>>();
    assert_eq!(vec_ext, vec_std);

    step = 3;
    let vec = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 3, 6, 8]);

    step = 4;
    let vec_ext = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    let vec_std = arr.into_iter().step_by(step).collect::<Vec<_>>();
    assert_eq!(vec_ext, vec_std);

    step = 5;
    let vec = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 5, 8]);

    step = 6;
    let vec = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 6, 8]);

    step = 7;
    let vec = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 7, 8]);

    step = 8;
    let vec = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 8]);

    step = 9;
    let vec = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 8]);

    step = 10;
    let vec = arr.into_iter().inclusive_step_by(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![0, 8]);
}

#[test]
fn test_inclusive_step_by_clone() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    let iter_ext = arr.iter().inclusive_step_by(1);
    let iter_ext_cloned = iter_ext.clone();
    let vec_ext = iter_ext.collect::<Vec<_>>();
    let vec_ext_cloned = iter_ext_cloned.collect::<Vec<_>>();
    assert_eq!(vec_ext, vec_ext_cloned);
}

#[test]
fn test_inclusive_step_by_len() {
    let arr: [u8; 0] = [];

    let mut step = 1;
    let vec = arr.iter().inclusive_step_by(step).len();
    assert_eq!(vec, 0);

    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    let vec_std_len = arr.iter().step_by(step).len();
    assert_eq!(vec_ext_len, vec_std_len);

    step = 2;
    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    let vec_std_len = arr.iter().step_by(step).len();
    assert_eq!(vec_ext_len, vec_std_len);

    step = 3;
    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    assert_eq!(vec_ext_len, 4);

    step = 4;
    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    let vec_std_len = arr.iter().step_by(step).len();
    assert_eq!(vec_ext_len, vec_std_len);

    step = 5;
    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    assert_eq!(vec_ext_len, 3);

    step = 6;
    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    assert_eq!(vec_ext_len, 3);

    step = 7;
    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    assert_eq!(vec_ext_len, 3);

    step = 8;
    let vec_ext_len = arr.iter().inclusive_step_by(step).len();
    assert_eq!(vec_ext_len, 2);
}

#[test]
fn test_step_boundary_len() {
    let arr: [u8; 0] = [];

    let mut step = 1;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 0);

    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    let vec_ext_len = arr.iter().step_boundary(step).len();
    let vec_std_len = arr.iter().step_by(step).len();
    assert_eq!(vec_ext_len, vec_std_len);

    step = 2;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 5);

    step = 3;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 3);

    step = 4;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 3);

    step = 5;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 2);

    step = 6;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 2);

    step = 7;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 2);

    step = 8;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 2);

    step = 9;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 1);

    step = 10;
    let vec = arr.iter().step_boundary(step).len();
    assert_eq!(vec, 1);
}

#[test]
fn test_step_boundary() {
    let arr: [u8; 0] = [];

    let mut step = 1;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![]);

    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    step = 1;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8)
        ]
    );

    step = 2;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 1), (2, 3), (4, 5), (6, 7), (8, 8)]);

    step = 3;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 2), (3, 5), (6, 8)]);

    step = 4;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 3), (4, 7), (8, 8)]);

    step = 5;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 4), (5, 8)]);

    step = 6;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 5), (6, 8)]);

    step = 7;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 6), (7, 8)]);

    step = 8;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 7), (8, 8)]);

    step = 9;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 8)]);

    step = 10;
    let vec = arr.into_iter().step_boundary(step).collect::<Vec<_>>();
    assert_eq!(vec, vec![(0, 8)]);
}

#[test]
fn test_to_range() {
    let arr_tup = [(0, 2), (3, 5), (6, 8), (7, 9)];

    let iter = arr_tup.into_iter().to_range();
    assert_eq!(iter.len(), 4);
    let vec_cloned = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec_cloned);
    assert_eq!(vec, vec![0..2, 3..5, 6..8, 7..9]);
}

#[test]
fn test_to_range_icv() {
    let arr_tup = [(0, 2), (3, 5), (6, 8), (7, 9), (10, 12)];

    let iter = arr_tup.iter().cloned().to_range_icv();
    assert_eq!(iter.len(), 5);
    let vec_cloned = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec_cloned);
    assert_eq!(vec, vec![0..=2, 3..=5, 6..=8, 7..=9, 10..=12]);
}

#[test]
fn test_to_tuple() {
    let arr_ran = [0..2, 3..5];

    let iter = arr_ran.into_iter().to_tuple();
    assert_eq!(iter.len(), 2);
    let vec_cloned = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec_cloned);
    assert_eq!(vec, vec![(0, 2), (3, 5)]);
}

#[test]
fn test_to_tuple_icv() {
    let arr_ran_icv = [0..=2, 3..=5, 6..=8];

    let iter = arr_ran_icv.into_iter().to_tuple_icv();
    assert_eq!(iter.len(), 3);
    let vec_cloned = iter.clone().collect::<Vec<_>>();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec_cloned);
    assert_eq!(vec, vec![(0, 2), (3, 5), (6, 8),]);
}
