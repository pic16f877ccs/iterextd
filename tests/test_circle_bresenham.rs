use iterextd::CircleBresenhamSeq;

#[test]
fn test_create_bresenham_circle_empty() {
    let circle = CircleBresenhamSeq::<i16>::new(0_u8);
    let vec = circle.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_create_bresenham_circle_r_two_clone() {
    let circle = CircleBresenhamSeq::<i64>::new(1_u32).clone();
    let vec = circle.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![ (0, -1), (1, 0), (0, 1), (-1, 0) ]
    );
}

#[test]
fn test_create_bresenham_circle_two_debug() {
    let circle = CircleBresenhamSeq::<i32>::new(2_u16);
    assert_eq!(format!("{:?}", circle), "CircleBresenhamSeq { x: -2, y: 0, err: -2, i: 1 }");
}

#[test]
fn test_create_bresenham_circle_r_three() {
    let circle = CircleBresenhamSeq::<i64>::new(3_u32);
    let vec = circle.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![
            (0, -3), (1, -3), (2, -2), (3, -1), (3, 0), (3, 1), (2, 2),
            (1, 3), (0, 3), (-1, 3), (-2, 2), (-3, 1), (-3, 0), (-3, -1),
            (-2, -2), (-1, -3) ]
    );
}

#[test]
fn test_create_bresenham_circle_r_five_rev() {
    let circle = CircleBresenhamSeq::<i128>::new(4_u64).rev();
    let vec = circle.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![
            (0, -4), (-1, -4), (-2, -3), (-3, -2), (-4, -1), (-4, 0),
            (-4, 1), (-3, 2), (-2, 3), (-1, 4), (0, 4), (1, 4), (2, 3),
            (3, 2), (4, 1), (4, 0), (4, -1), (3, -2), (2, -3), (1, -4) ]
    );
}


#[test]
fn test_create_bresenham_circle_r_one_type_u8() {
    let circle = CircleBresenhamSeq::<i64>::new(1_u8);
    let vec = circle.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![ (0, -1), (1, 0), (0, 1), (-1, 0) ]
    );
}

#[test]
fn test_create_bresenham_circle_r_one_type_u16() {
    let circle = CircleBresenhamSeq::<i64>::new(1_u16);
    let vec = circle.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![ (0, -1), (1, 0), (0, 1), (-1, 0) ]
    );
}

