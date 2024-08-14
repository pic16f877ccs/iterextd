use iterextd::TupleIter;

// Helper function to compare tuples for equality
fn assert_tuple_eq<T: PartialEq + std::fmt::Debug>(actual: &T, expected: &T) {
    assert_eq!(actual, expected);
}

#[test]
fn test_tuple_iterator_str() {
    let tup = ("hello", "world");
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&"hello"));
    assert_eq!(tup_iter.next(), Some(&"world"));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_string() {
    let tup = (String::from("foo"), String::from("bar"));
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&String::from("foo")));
    assert_eq!(tup_iter.next(), Some(&String::from("bar")));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_numbers() {
    let tup = (1.5, 2.5, -3.5);
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&1.5));
    assert_eq!(tup_iter.next(), Some(&2.5));
    assert_eq!(tup_iter.next(), Some(&-3.5));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_vec() {
    let tup = (vec![1, 2, 3], vec![4, 5, 6]);
    let mut tup_iter = tup.tuple_iter();
    assert_tuple_eq(tup_iter.next().unwrap(), &vec![1, 2, 3]);
    assert_tuple_eq(tup_iter.next().unwrap(), &vec![4, 5, 6]);
    assert_eq!(tup_iter.next(), None);
}

#[derive(Debug, PartialEq)]
enum TestEnum {
    VariantA,
    VariantB(i32),
    VariantC(String),
}

#[test]
fn test_tuple_iterator_enum() {
    let tup = (
        TestEnum::VariantA,
        TestEnum::VariantB(42),
        TestEnum::VariantC(String::from("enum")),
    );
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&TestEnum::VariantA));
    assert_eq!(tup_iter.next(), Some(&TestEnum::VariantB(42)));
    assert_eq!(
        tup_iter.next(),
        Some(&TestEnum::VariantC(String::from("enum")))
    );
    assert_eq!(tup_iter.next(), None);
}

#[derive(Debug, PartialEq)]
struct TestStruct {
    field1: i32,
    field2: String,
}

#[test]
fn test_tuple_iterator_struct() {
    let tup = (
        TestStruct {
            field1: 10,
            field2: String::from("struct"),
        },
        TestStruct {
            field1: 20,
            field2: String::from("tuple"),
        },
    );
    let mut tup_iter = tup.tuple_iter();
    assert_tuple_eq(
        tup_iter.next().unwrap(),
        &TestStruct {
            field1: 10,
            field2: String::from("struct"),
        },
    );
    assert_tuple_eq(
        tup_iter.next().unwrap(),
        &TestStruct {
            field1: 20,
            field2: String::from("tuple"),
        },
    );
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_mut_str_mutate() {
    let mut tup = ("hello", "world");
    let mut tup_iter = tup.tuple_iter_mut();

    if let Some(mut_ref) = tup_iter.next() {
        *mut_ref = "new_hello";
    }

    if let Some(mut_ref) = tup_iter.next() {
        *mut_ref = "new_world";
    }

    assert_eq!(tup, ("new_hello", "new_world"));
}

#[test]
fn test_tuple_iterator_mut_string_mutate() {
    let mut tup = (String::from("foo"), String::from("bar"));
    let mut tup_iter = tup.tuple_iter_mut();

    if let Some(mut_ref) = tup_iter.next() {
        mut_ref.push_str("_appended");
    }

    if let Some(mut_ref) = tup_iter.next() {
        mut_ref.push_str("_appended");
    }

    assert_eq!(
        tup,
        (String::from("foo_appended"), String::from("bar_appended"))
    );
}

#[test]
fn test_tuple_iterator_mut_numbers_mutate() {
    let mut tup = (1.0, 2.5, -3.0);
    let mut tup_iter = tup.tuple_iter_mut();

    if let Some(mut_ref) = tup_iter.next() {
        *mut_ref += 10.0;
    }

    if let Some(mut_ref) = tup_iter.next() {
        *mut_ref *= 2.0;
    }

    if let Some(mut_ref) = tup_iter.next() {
        *mut_ref = 42.0;
    }

    assert_eq!(tup, (11.0, 5.0, 42.0));
}

#[test]
fn test_tuple_iterator_mut_vec_mutate() {
    let mut tup = (vec![1, 2, 3], vec![4, 5, 6]);
    let mut tup_iter = tup.tuple_iter_mut();

    if let Some(mut_ref) = tup_iter.next() {
        mut_ref.push(4);
    }

    if let Some(mut_ref) = tup_iter.next() {
        mut_ref.pop();
    }

    assert_eq!(tup, (vec![1, 2, 3, 4], vec![4, 5]));
}

#[test]
fn test_tuple_iterator_mut_enum_mutate() {
    let mut tup = (
        TestEnum::VariantA,
        TestEnum::VariantB(42),
        TestEnum::VariantC(String::from("enum")),
    );
    let mut tup_iter = tup.tuple_iter_mut();

    if let Some(mut_ref) = tup_iter.next() {
        *mut_ref = TestEnum::VariantB(99);
    }

    if let Some(mut_ref) = tup_iter.next() {
        if let TestEnum::VariantB(value) = mut_ref {
            *value += 1;
        }
    }

    if let Some(mut_ref) = tup_iter.next() {
        if let TestEnum::VariantC(string) = mut_ref {
            string.push_str("_modified");
        }
    }

    assert_eq!(
        tup,
        (
            TestEnum::VariantB(99),
            TestEnum::VariantB(43),
            TestEnum::VariantC(String::from("enum_modified"))
        )
    );
}

#[test]
fn test_tuple_iterator_mut_struct_mutate() {
    let mut tup = (
        TestStruct {
            field1: 10,
            field2: String::from("struct"),
        },
        TestStruct {
            field1: 20,
            field2: String::from("tuple"),
        },
    );
    let mut tup_iter = tup.tuple_iter_mut();

    if let Some(mut_ref) = tup_iter.next() {
        mut_ref.field1 += 5;
        mut_ref.field2.push_str("_modified");
    }

    if let Some(mut_ref) = tup_iter.next() {
        mut_ref.field1 *= 2;
        mut_ref.field2.push_str("_modified");
    }

    assert_eq!(
        tup,
        (
            TestStruct {
                field1: 15,
                field2: String::from("struct_modified")
            },
            TestStruct {
                field1: 40,
                field2: String::from("tuple_modified")
            }
        )
    );
}

#[test]
fn test_tuple_iterator_double_ended_str() {
    let tup = ("hello", "world");
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&"hello"));
    assert_eq!(tup_iter.next_back(), Some(&"world"));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_double_ended_string() {
    let tup = (String::from("foo"), String::from("bar"));
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&String::from("foo")));
    assert_eq!(tup_iter.next_back(), Some(&String::from("bar")));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_double_ended_numbers() {
    let tup = (1.0, 2.5, -3.0);
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&1.0));
    assert_eq!(tup_iter.next_back(), Some(&-3.0));
    assert_eq!(tup_iter.next(), Some(&2.5));
    assert_eq!(tup_iter.next_back(), None);
}

#[test]
fn test_tuple_iterator_double_ended_vec() {
    let tup = (vec![1, 2, 3], vec![4, 5, 6]);
    let mut tup_iter = tup.tuple_iter();
    assert_tuple_eq(tup_iter.next().unwrap(), &vec![1, 2, 3]);
    assert_tuple_eq(tup_iter.next_back().unwrap(), &vec![4, 5, 6]);
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_double_ended_enum() {
    let tup = (
        TestEnum::VariantA,
        TestEnum::VariantB(42),
        TestEnum::VariantC(String::from("enum")),
    );
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&TestEnum::VariantA));
    assert_eq!(
        tup_iter.next_back(),
        Some(&TestEnum::VariantC(String::from("enum")))
    );
    assert_eq!(tup_iter.next(), Some(&TestEnum::VariantB(42)));
    assert_eq!(tup_iter.next_back(), None);
}

#[test]
fn test_tuple_iterator_double_ended_struct() {
    let tup = (
        TestStruct {
            field1: 10,
            field2: String::from("struct"),
        },
        TestStruct {
            field1: 20,
            field2: String::from("tuple"),
        },
    );
    let mut tup_iter = tup.tuple_iter();
    assert_tuple_eq(
        tup_iter.next().unwrap(),
        &TestStruct {
            field1: 10,
            field2: String::from("struct"),
        },
    );
    assert_tuple_eq(
        tup_iter.next_back().unwrap(),
        &TestStruct {
            field1: 20,
            field2: String::from("tuple"),
        },
    );
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_exact_size_str() {
    let tup = ("hello", "world");
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);

    let tuple = (1, 2, 3, 4, 5);
    let mut tuple_iter = tuple.tuple_iter();
    assert_eq!(tuple_iter.len(), 5);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 4);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 3);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 2);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 1);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 0);
}

#[test]
fn test_tuple_iterator_exact_size_string() {
    let tup = (String::from("foo"), String::from("bar"));
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_iterator_exact_size_numbers() {
    let tup = (1.0, 2.5, -3.0);
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.len(), 3);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_iterator_exact_size_vec() {
    let tup = (vec![1, 2, 3], vec![4, 5, 6]);
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_iterator_exact_size_enum() {
    let tup = (
        TestEnum::VariantA,
        TestEnum::VariantB(42),
        TestEnum::VariantC(String::from("enum")),
    );
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.len(), 3);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_iterator_exact_size_struct() {
    let tup = (
        TestStruct {
            field1: 10,
            field2: String::from("struct"),
        },
        TestStruct {
            field1: 20,
            field2: String::from("tuple"),
        },
    );
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_iter_reference() {
    let num = 5;
    let tup = (&num,);
    let mut iter = tup.tuple_iter();
    assert_eq!(iter.next(), Some(&&5));
    assert_eq!(iter.next(), None);
}

// Test cases for tuples of different lengths
#[test]
fn test_tuple_iterator_length_1() {
    let tup = (42,);
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&42));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_length_2() {
    let tup = (true, false);
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&true));
    assert_eq!(tup_iter.next(), Some(&false));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_length_3() {
    let tup = ('a', 'b', 'c');
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&'a'));
    assert_eq!(tup_iter.next(), Some(&'b'));
    assert_eq!(tup_iter.next(), Some(&'c'));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_length_4() {
    let tup = ('a', 'b', 'c', 'd');
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&'a'));
    assert_eq!(tup_iter.next(), Some(&'b'));
    assert_eq!(tup_iter.next(), Some(&'c'));
    assert_eq!(tup_iter.next(), Some(&'d'));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iterator_length_5() {
    let tup = ("a", "b", "c", "d", "e");
    let mut tup_iter = tup.tuple_iter();
    assert_eq!(tup_iter.next(), Some(&"a"));
    assert_eq!(tup_iter.next(), Some(&"b"));
    assert_eq!(tup_iter.next(), Some(&"c"));
    assert_eq!(tup_iter.next(), Some(&"d"));
    assert_eq!(tup_iter.next(), Some(&"e"));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_iter_12() {
    let tup = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    let mut iter = tup.tuple_iter();
    for i in 0..12 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_iter_11() {
    let tup = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let mut iter = tup.tuple_iter();
    for i in 0..11 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_iter_10() {
    let tup = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    let mut iter = tup.tuple_iter();
    for i in 0..10 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_iter_9() {
    let tup = (0, 1, 2, 3, 4, 5, 6, 7, 8);
    let mut iter = tup.tuple_iter();
    for i in 0..9 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_iter_8() {
    let tup = (0, 1, 2, 3, 4, 5, 6, 7);
    let mut iter = tup.tuple_iter();
    for i in 0..8 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_iter_7() {
    let tup = (0, 1, 2, 3, 4, 5, 6);
    let mut iter = tup.tuple_iter();
    for i in 0..7 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_iter_6() {
    let tup = (0.0f64, 1.0, 2.0, 3.0, 4.0, 5.0);
    let mut iter = tup.tuple_iter().copied();
    for i in 0..6 {
        assert_eq!(iter.next(), Some(i as f64));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_iter_5() {
    let tup = (0, 1, 2, 3, 4);
    let mut iter = tup.tuple_iter();
    for i in 0..5 {
        assert_eq!(iter.next(), Some(&i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iterator_length_1() {
    let mut tup = (42,);
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.next(), Some(&mut 42));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_mut_iterator_length_2() {
    let mut tup = (true, false);
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.next(), Some(&mut true));
    assert_eq!(tup_iter.next(), Some(&mut false));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_mut_iterator_length_3() {
    let mut tup = ('a', 'b', 'c');
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.next(), Some(&mut 'a'));
    assert_eq!(tup_iter.next(), Some(&mut 'b'));
    assert_eq!(tup_iter.next(), Some(&mut 'c'));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_mut_iterator_length_4() {
    let mut tup = ('a', 'b', 'c', 'd');
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.next(), Some(&mut 'a'));
    assert_eq!(tup_iter.next(), Some(&mut 'b'));
    assert_eq!(tup_iter.next(), Some(&mut 'c'));
    assert_eq!(tup_iter.next(), Some(&mut 'd'));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_mut_iterator_length_5() {
    let mut tup = ("a", "b", "c", "d", "e");
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.next(), Some(&mut "a"));
    assert_eq!(tup_iter.next(), Some(&mut "b"));
    assert_eq!(tup_iter.next(), Some(&mut "c"));
    assert_eq!(tup_iter.next(), Some(&mut "d"));
    assert_eq!(tup_iter.next(), Some(&mut "e"));
    assert_eq!(tup_iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_12() {
    let mut tup = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..12 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_11() {
    let mut tup = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..11 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_10() {
    let mut tup = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..10 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_9() {
    let mut tup = (0, 1, 2, 3, 4, 5, 6, 7, 8);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..9 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_8() {
    let mut tup = (0, 1, 2, 3, 4, 5, 6, 7);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..8 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_7() {
    let mut tup = (0, 1, 2, 3, 4, 5, 6);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..7 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_6() {
    let mut tup = (0, 1, 2, 3, 4, 5);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..6 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iter_5() {
    let mut tup = (0, 1, 2, 3, 4);
    let mut iter = tup.tuple_iter_mut();
    for mut i in 0..5 {
        assert_eq!(iter.next(), Some(&mut i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_tuple_mut_iterator_exact_size_str() {
    let mut tup = ("hello", "world");
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);

    let mut tuple = (1, 2, 3, 4, 5);
    let mut tuple_iter = tuple.tuple_iter_mut();
    assert_eq!(tuple_iter.len(), 5);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 4);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 3);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 2);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 1);
    let _ = tuple_iter.next();
    assert_eq!(tuple_iter.len(), 0);
}

#[test]
fn test_tuple_mut_iterator_exact_size_string() {
    let mut tup = (String::from("foo"), String::from("bar"));
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_mut_iterator_exact_size_numbers() {
    let mut tup = (1.0, 2.5, -3.0);
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.len(), 3);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_mut_iterator_exact_size_vec() {
    let mut tup = (vec![1, 2, 3], vec![4, 5, 6]);
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_mut_iterator_exact_size_enum() {
    let mut tup = (
        TestEnum::VariantA,
        TestEnum::VariantB(42),
        TestEnum::VariantC(String::from("enum")),
    );
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.len(), 3);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}

#[test]
fn test_tuple_mut_iterator_exact_size_struct() {
    let mut tup = (
        TestStruct {
            field1: 10,
            field2: String::from("struct"),
        },
        TestStruct {
            field1: 20,
            field2: String::from("tuple"),
        },
    );
    let mut tup_iter = tup.tuple_iter_mut();
    assert_eq!(tup_iter.len(), 2);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 1);
    let _ = tup_iter.next();
    assert_eq!(tup_iter.len(), 0);
}
