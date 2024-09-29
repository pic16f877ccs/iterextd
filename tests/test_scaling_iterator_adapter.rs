use iterextd::Scaler;
use num_convert::FromByAdd;
use paste::paste;

macro_rules! test_scaling_unsigned_type {
        ( $tmp_type:ty, $input_type:ty; $($output_type:ty),* ) => {
            $(
                paste! {
                    #[test]
                    fn [<test_uintermediate_$input_type _$tmp_type _$output_type _to_otput_type_hall_size>]() {
                        let iter = [<$input_type>::MIN, (<$tmp_type>::MAX / 2) as $input_type]
                            .into_iter().scaling::<$tmp_type>(<$output_type>::MIN..=<$output_type>::MIN + 2);
                        let vec = iter.collect::<Vec<_>>();
                        assert_eq!(vec, vec![<$output_type>::MIN, <$output_type>::MIN + 2]);
                    }
                    #[test]
                    #[should_panic]
                    fn [<test_uintermediate_$input_type _$tmp_type _$output_type _to_otput_type_err>]() {
                        let _iter = [<$input_type>::MIN, (<$tmp_type>::MAX / 2 + 1) as $input_type]
                            .into_iter().scaling::<$tmp_type>(<$output_type>::MIN..=<$output_type>::MIN + 2);
                    }
                }
            )*
        }
    }

macro_rules! test_scaling_type {
        ( $tmp_type:ty, $input_type:ty; $($output_type:ty),* ) => {
            $(
                paste! {
                    #[test]
                    fn [<test_uintermediate_$input_type _$tmp_type _$output_type _to_otput_type>]() {
                        let iter = [<$input_type>::MIN, <$input_type>::MIN + 1, <$input_type>::MIN + 2]
                            .into_iter().scaling::<$tmp_type>(<$output_type>::MIN..=<$output_type>::MIN + 2);
                        let vec = iter.collect::<Vec<_>>();
                        assert_eq!(vec, vec![<$output_type>::MIN, <$output_type>::MIN + 1, <$output_type>::MIN + 2]);
                    }
                }
            )*
        }
    }

#[test]
fn test_scaling_debug() {
    let iter = [0u8, 1, 2].into_iter().scaling::<u16>(0..=25u8);
    assert_eq!(
        format!("{:?}", iter),
        "Scaling { denominator: 2, numerator: 25, input_start: 0, \
        output_start: 0, iter: IntoIter([0, 1, 2]), phantom: PhantomData<u8> }"
    );
}

#[test]
fn test_scaling_empty() {
    let arr: [u8; 0] = [];
    let iter = arr.into_iter().scaling::<u16>(100..=255u16);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![]);
}

#[test]
fn test_scaling_one_element_min_val_offset() {
    let iter = [i8::MIN].into_iter().scaling::<u16>(255..=255u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![255u8]);

    let iter = [i8::MIN].into_iter().scaling::<u16>(5..=255u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![5u8]);

    let iter = [i8::MIN].into_iter().scaling::<u16>(0..=255u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![0u8]);

    let iter = [i8::MIN].into_iter().scaling::<u16>(i8::MIN..=i8::MIN);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![i8::MIN]);

    let iter = [i16::MIN].into_iter().scaling::<u16>(-5..=5i8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![-5i8]);

    let iter = [i16::MIN].into_iter().scaling::<u32>(u16::MAX..=u16::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![u16::MAX]);

    let iter = [i16::MIN].into_iter().scaling::<u16>(i8::MIN..=i8::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![i8::MIN]);

    let iter = [i16::MIN].into_iter().scaling::<u16>(0..=i8::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![0]);
}

#[test]
fn test_scaling_one_element_zero() {
    let iter = [0u8].into_iter().scaling::<u16>(0..=0u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![0u8]);

    let iter = [0u8].into_iter().scaling::<u16>(5..=255u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![5u8]);

    let iter = [0u8].into_iter().scaling::<u32>(5..=u16::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![5u16]);

    let iter = [0u16].into_iter().scaling::<u16>(5..=255u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![5u8]);

    let iter = [0i8].into_iter().scaling::<u32>(256..=u16::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![u16::MAX]);

    let iter = [0i16].into_iter().scaling::<u64>(255..=255u16);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![255]);

    let iter = [0i8].into_iter().scaling::<u32>(-500..=i16::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![i16::MAX]);

    let iter = [0i32].into_iter().scaling::<u64>(255..=255u16);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![255]);
}

#[test]
fn test_scaling_one_elements_bounds() {
    let iter = [0u8].into_iter().scaling::<u32>(..u8::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![0u8]);

    let iter = [i8::MAX].into_iter().scaling::<u16>(..=i8::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![i8::MAX]);

    let iter = [i8::MAX].into_iter().scaling::<u16>(..=u8::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![u8::MAX]);

    let iter = [i8::MIN].into_iter().scaling::<u16>(..=i8::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![i8::MIN]);

    let iter = [i8::MAX].into_iter().scaling::<u16>(0i8..);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![i8::MAX]);

    let iter = [0u8].into_iter().scaling::<u32>(10u8..);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![10u8]);

    let iter = [i8::MAX].into_iter().scaling::<u16>(..);
    let result = iter.collect::<Vec<i8>>();
    assert_eq!(result, vec![i8::MAX]);

    let iter = [0u8].into_iter().scaling::<u32>(..);
    let result = iter.collect::<Vec<u8>>();
    assert_eq!(result, vec![0u8]);

    let iter = [0u8].into_iter().scaling::<u16>(0..);
    let result = iter.collect::<Vec<i8>>();
    assert_eq!(result, vec![0i8]);

    let iter = [u16::MAX].into_iter().scaling::<u32>(0..);
    let result = iter.collect::<Vec<i16>>();
    assert_eq!(result, vec![i16::MAX]);

    let iter = [u32::MAX as usize]
        .into_iter()
        .scaling::<usize>(0..=u32::MAX as usize + 2);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![u32::MAX as usize + 2]);
}

#[test]
fn test_scaling_three_elements() {
    let iter = [0usize, 0, 0].into_iter().scaling::<usize>(0..=0usize);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![0, 0, 0]);

    let iter = [0usize, 0, 0].into_iter().scaling::<usize>(10..=20usize);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![10, 10, 10]);

    let iter = [1u32, 1, 1].into_iter().scaling::<u32>(0..=2u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![2, 2, 2]);

    let iter = [1u32, 1, 1].into_iter().scaling::<u32>(5..=20u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![20, 20, 20]);

    let iter = [0i16, 0, 0].into_iter().scaling::<u32>(0..=2u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![2, 2, 2]);

    let iter = [0u32, 1, 2].into_iter().scaling::<u32>(..=20u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![0, 10, 20]);

    let iter = [0u32, 1, 2].into_iter().scaling::<u32>(3..=20u8);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![3, 11, 20]);

    let iter = [i16::MIN, i16::MIN, i16::MIN]
        .into_iter()
        .scaling::<u32>(..=i8::MAX);
    let result = iter.collect::<Vec<_>>();
    assert_eq!(result, vec![i8::MIN, i8::MIN, i8::MIN]);
}

#[test]
fn test_scaling_range_tup_in_out() {
    let in_range = i8::MIN..=i8::MAX;
    let iter = in_range.clone().scaling::<u16>((i8::MIN, i8::MAX));
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);
}

#[test]
#[should_panic]
fn test_scaling_range_in_out_panic() {
    let in_range = i8::MIN..=i8::MAX;
    let _iter = in_range.scaling::<u16>(i8::MAX..=i8::MIN);
}

#[test]
#[should_panic]
fn test_scaling_range_tup_in_panic() {
    let in_range = i8::MIN..=i8::MAX;
    let _iter = in_range.scaling::<u16>((i8::MAX, i8::MIN));
}

#[test]
fn test_scaling_ranges_in_out() {
    let in_range = i8::MIN..=i8::MAX;
    let iter = in_range.clone().scaling::<u16>(..);
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);

    let in_range = u8::MIN..=u8::MAX;
    let iter = in_range.clone().scaling::<u16>(..);
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);

    let in_range = i8::MIN..=i8::MAX;
    let iter = in_range.clone().scaling::<u16>(..=i8::MAX);
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);

    let in_range = u8::MIN..=u8::MAX;
    let iter = in_range.clone().scaling::<u16>(..=u8::MAX);
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);

    let in_range = u8::MIN..=u8::MAX;
    let iter = in_range.clone().scaling::<u16>(i8::MIN..);
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.map(|x| u8::from_by_add(x)).collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);

    let in_range = u8::MIN..=u8::MAX;
    let iter = in_range.clone().scaling::<u16>(u8::MIN..);
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.map(|x| u8::from_by_add(x)).collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);

    let in_range = i16::MIN..=i16::MAX;
    let iter = in_range.clone().scaling::<u64>(..=i16::MAX);
    let result_in_range = in_range.collect::<Vec<_>>();
    let result_out_range = iter.collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);
}

#[test]
fn test_scaling_size_hint() {
    let mut range = i8::MIN..=i8::MAX;
    let _ = range.by_ref().nth(10);
    let mut iter = range.clone().scaling::<u16>(..=u8::MAX);
    assert_eq!(range.size_hint(), iter.size_hint());
    let _ = range.nth(5);
    let _ = iter.nth(5);
    assert_eq!(range.size_hint(), iter.size_hint());
}

#[test]
fn test_scaling_rev() {
    let in_range = i8::MIN..=i8::MAX;
    let iter = in_range.clone().scaling::<u16>(..=i8::MAX);
    let result_in_range = in_range.rev().collect::<Vec<_>>();
    let result_out_range = iter.rev().collect::<Vec<_>>();
    assert_eq!(result_in_range, result_out_range);
}

#[test]
fn test_scaling_len() {
    let in_range = i8::MIN..=i8::MAX;
    let iter = in_range.clone().scaling::<u16>(..=i8::MAX);
    assert_eq!(in_range.len(), iter.len());
}

test_scaling_type!(u8,  u8; u8, i8, u16, u32, u64, usize, u128);
test_scaling_type!(u16, u8; u8, i8, u16, i16, u32, u64, usize, u128);
test_scaling_type!(u32, u8; u8, i8, u16, i16, u32, i32, u64, usize, u128);
test_scaling_type!(u64, u8; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, u8; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, u8; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u8,  i8; u8, i8, u16, u32, u64, usize, u128);
test_scaling_type!(u16, i8; u8, i8, u16, i16, u32, u64, usize, u128);
test_scaling_type!(u32, i8; u8, i8, u16, i16, u32, i32, u64, usize, u128);
test_scaling_type!(u64, i8; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, i8; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, i8; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u8,  u16; u8, i8, u16, u32, u64, usize, u128);
test_scaling_type!(u16, u16; u8, i8, u16, i16, u32, u64, usize, u128);
test_scaling_type!(u32, u16; u8, i8, u16, i16, u32, i32, u64, usize, u128);
test_scaling_type!(u64, u16; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, u16; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, u16; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u16, i16; u8, i8, u16, i16, u32, u64, usize, u128);
test_scaling_type!(u32, i16; u8, i8, u16, i16, u32, i32, u64, usize, u128);
test_scaling_type!(u64, i16; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, i16; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, i16; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u32, u32; u8, i8, u16, i16, u32, i32, u64, usize, u128);
test_scaling_type!(u64, u32; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, u32; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, u32; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u32, i32; u8, i8, u16, i16, u32, i32, u64, usize, u128);
test_scaling_type!(u64, i32; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, i32; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, i32; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u64, i64; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, i64; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, i64; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u64, usize; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, usize; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, usize; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u64, isize; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(usize, isize; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128);
test_scaling_type!(u128, isize; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u128, i128; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_type!(u128, u128; u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, u128, i128);

test_scaling_unsigned_type!(u8, u8; u8, u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u8, u16; u8, u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u8, u32; u8, u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u8, u64; u8, u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u8, usize; u8, u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u8, u128; u8, u16, u32, u64, usize, u128);

test_scaling_unsigned_type!(u16, u16;   u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u16, u32;   u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u16, u64;   u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u16, usize; u16, u32, u64, usize, u128);
test_scaling_unsigned_type!(u16, u128;  u16, u32, u64, usize, u128);

test_scaling_unsigned_type!(u32, u32;   u32, u64, usize, u128);
test_scaling_unsigned_type!(u32, u64;   u32, u64, usize, u128);
test_scaling_unsigned_type!(u32, usize; u32, u64, usize, u128);
test_scaling_unsigned_type!(u32, u128;  u32, u64, usize, u128);

test_scaling_unsigned_type!(u64, u64;   u64, usize, u128);
test_scaling_unsigned_type!(u64, usize; u64, usize, u128);
test_scaling_unsigned_type!(u64, u128;  u64, usize, u128);

test_scaling_unsigned_type!(usize, u64;   u64, usize, u128);
test_scaling_unsigned_type!(usize, usize; u64, usize, u128);
test_scaling_unsigned_type!(usize, u128;  u64, usize, u128);

test_scaling_unsigned_type!(u128, u128;  u128);
