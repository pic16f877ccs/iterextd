use iterextd::{SwapIter, TupleIter};

#[test]
fn test_swap_iter() {
    let mut first_vec = (0, 1, 2, 3, 4, 5, 6, 7);
    let mut second_vec = vec![10, 11, 12, 13, 14, 15];
    let first_iter = first_vec.tuple_iter_mut().step_by(2);
    let second_iter = second_vec.iter_mut().step_by(2);
    let _ = first_iter.swap_elems(second_iter);
    assert_eq!(first_vec, (10, 1, 12, 3, 14, 5, 6, 7));
    assert_eq!(second_vec, [0, 11, 2, 13, 4, 15]);
}
