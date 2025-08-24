use iterextd::Iter2D;

#[test]
fn test_overlay_2d() {
    let vec = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 0, 0, 0, 0,
        0, 0, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 39, 40, 41, 42, 0, 0, 0, 0, 0, 0, 49, 50, 51, 52,
        0, 0, 0, 0, 0, 0, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0, 69, 70, 71, 72, 0, 0, 0, 0, 0, 0, 79,
        80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
    ];

    let other_vec = [
        23, 24, 25, 26, 27, 28, 33, 34, 35, 36, 37, 38, 43, 44, 45, 46, 47, 48, 53, 54, 55, 56, 57,
        58, 63, 64, 65, 66, 67, 68, 73, 74, 75, 76, 77, 78,
    ];

    let res_vec = vec
        .iter()
        .take(80)
        .overlay_2d(
            other_vec.iter(),
            (10, 8), // target size
            (6, 6),  // source size
            (2, 2),  // offsets
        )
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(res_vec, (1..=80).collect::<Vec<_>>());

    let cloned_vec = vec.clone();
    let cloned_other_vec = other_vec.clone();
    let res_vec = cloned_vec
        .into_iter()
        .overlay_2d(
            cloned_other_vec.into_iter(), // other data
            (10, 10),                     // target size
            (6, 6),                       // source size
            (2, 2),                       // offsets
        )
        .collect::<Vec<_>>();
    assert_eq!(res_vec, (1..=100).collect::<Vec<_>>());

    let res_vec = vec
        .into_iter()
        .overlay_2d(
            other_vec.iter().cloned(), // other data
            (10, 10),                  // target size
            (6, 6),                    // source size
            (2, 2),                    // offsets
        )
        .collect::<Vec<_>>();
    assert_eq!(res_vec, (1..=100).collect::<Vec<_>>());

    assert_eq!(
        (1..=100)
            .overlay_2d(
                0..0,
                (10, 10), // target size
                (6, 6),   // source size
                (2, 2),   // offsets
            )
            .collect::<Vec<_>>(),
        (1..=100).collect::<Vec<_>>()
    );

    assert_eq!(
        (1..=100)
            .overlay_2d(
                101..=200,
                (10, 10), // target size
                (10, 10), // source size
                (0, 0),   // offsets
            )
            .collect::<Vec<_>>(),
        (101..=200).collect::<Vec<_>>()
    );

    assert_eq!(
        (0..0)
            .overlay_2d(
                10..=20,
                (10, 10), // target size
                (10, 10), // source size
                (0, 0),   // offsets
            )
            .collect::<Vec<_>>(),
        (0..0).collect::<Vec<_>>()
    );

    let right_iter = (1..=100).overlay_2d(
        1..=64,
        (10, 10), // target size
        (8, 8),   // source size
        (1, 1),   // offsets
    );

    let left_iter = right_iter.clone();

    assert_eq!(
        left_iter.collect::<Vec<_>>(),
        right_iter.collect::<Vec<_>>()
    );
}

#[test]
#[should_panic(expected = "the offset and size of the inserted abstract 2D data exceed the bounds")]
fn test_overlay_2d_panic_width_offset() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (10, 10), // target size
        (4, 10),  // source size
        (7, 0),   // offsets
    );
}

#[test]
#[should_panic(expected = "the offset and size of the inserted abstract 2D data exceed the bounds")]
fn test_overlay_2d_panic_height_offset() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (10, 10), // target size
        (3, 10),  // source size
        (7, 1),   // offsets
    );
}

#[test]
#[should_panic(expected = "the size of abstract 2D data must not be zero")]
fn test_overlay_2d_panic_size_height_zero() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (10, 0), // target size
        (3, 0),  // source size
        (7, 0),  // offsets
    );
}

#[test]
#[should_panic(expected = "the size of abstract 2D data must not be zero")]
fn test_overlay_2d_panic_size_width_zero() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (0, 10), // target size
        (0, 10), // source size
        (0, 0),  // offsets
    );
}

#[test]
#[should_panic(expected = "the size of abstract 2D data must not be zero")]
fn test_overlay_2d_panic_width_zero() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (0, 10), // target size
        (3, 4),  // source size
        (7, 1),  // offsets
    );
}

#[test]
#[should_panic(expected = "the size of abstract 2D data must not be zero")]
fn test_overlay_2d_panic_height_zero() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (10, 0), // target size
        (3, 10), // source size
        (7, 1),  // offsets
    );
}

#[test]
#[should_panic(expected = "the size of abstract 2D data must not be zero")]
fn test_overlay_2d_panic_height_zero_other() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (10, 10), // target size
        (3, 0),   // source size
        (7, 1),   // offsets
    );
}

#[test]
#[should_panic(expected = "the size of abstract 2D data must not be zero")]
fn test_overlay_2d_panic_width_zero_other() {
    let _ = (1..100).overlay_2d(
        1..=40,
        (10, 10), // target size
        (0, 5),   // source size
        (7, 1),   // offsets
    );
}

#[test]
fn test_overlay_2d_overlay_iter_longer_than_overlay_size() {
    let base = vec![0; 9];
    let overlay = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let result: Vec<_> = base
        .into_iter()
        .overlay_2d(overlay.into_iter(), (3, 3), (2, 2), (1, 1))
        .collect();

    assert_eq!(result.len(), 9);
}

#[test]
fn test_overlay_2d_overlay_iter_shorter_than_overlay_size() {
    let base = vec![0; 9];
    let overlay = vec![1, 2];
    let result: Vec<_> = base
        .into_iter()
        .overlay_2d(overlay.into_iter(), (3, 3), (2, 2), (1, 1))
        .collect();

    assert_eq!(result.len(), 9);
}

#[test]
fn test_overlay_2d_overlay_fits_exactly_at_bottom_right() {
    // Overlay is inserted in the bottom right corner of base
    let base = vec![0; 16];
    let overlay = vec![1, 2, 3, 4];
    let result: Vec<_> = base.into_iter().overlay_2d(
        overlay.into_iter(),
        (4, 4),
        (2, 2),
        (2, 2),
    ).collect();
    // Overlay must replace the last 2 rows, last 2 colunns
    assert_eq!(
        result,
        vec![
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 1, 2,
            0, 0, 3, 4,
        ]
    );
}

#[test]
fn test_overlay_2d_overlay_single_cell() {
    // Overlay size 1x1 in center of base
    let base = vec![0; 9];
    let overlay = vec![9];
    let result: Vec<_> = base.into_iter().overlay_2d(
        overlay.into_iter(),
        (3, 3),
        (1, 1),
        (1, 1),
    ).collect();
    assert_eq!(
        result,
        vec![
            0, 0, 0,
            0, 9, 0,
            0, 0, 0,
        ]
    );
}

#[test]
fn test_overlay_2d_overlay_full_row() {
    // Overlay size 3x1 (one line)
    let base = vec![0; 9];
    let overlay = vec![1, 2, 3];
    let result: Vec<_> = base.into_iter().overlay_2d(
        overlay.into_iter(),
        (3, 3),
        (3, 1),
        (0, 1),
    ).collect();
    assert_eq!(
        result,
        vec![
            0, 0, 0,
            1, 2, 3,
            0, 0, 0,
        ]
    );
}

#[test]
fn test_overlay_2d_overlay_full_column() {
    // Overlay size 1x3 (one row)
    let base = vec![0; 9];
    let overlay = vec![1, 2, 3];
    let result: Vec<_> = base.into_iter().overlay_2d(
        overlay.into_iter(),
        (3, 3),
        (1, 3),
        (1, 0),
    ).collect();
    assert_eq!(
        result,
        vec![
            0, 1, 0,
            0, 2, 0,
            0, 3, 0,
        ]
    );
}

#[test]
fn test_overlay_2d_overlay_len_size_hint() {
    // Overlay len 90 elements
    let iter = (1..101).take(90).overlay_2d(1..65, (10, 10), (8, 8), (1, 1));
    assert_eq!(iter.size_hint().0, 90);
    assert_eq!(iter.size_hint().1, Some(90));
    assert_eq!(iter.len(), 90);
}

#[test]
fn test_overlay_2d_debug() {
    // Check that the debug output is correct
    let iter = (1..=100).overlay_2d(1..=64, (10, 10), (8, 8), (1, 1));

    let debug_str = "Overlay2D { base_iter: 1..=100, overlay_iter: 1..=64, \
        base_index: 0, overlay_index: 11, overlay_row_width: 8, \
        overlay_row_padding: 2, in_overlay_row: false }";  
    assert_eq!(format!("{:?}", iter), debug_str);
}
