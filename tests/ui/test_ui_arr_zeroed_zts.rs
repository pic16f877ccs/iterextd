use iterextd::IterExtd;

fn main() {
    let mut arr = [10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222];
    let arr_sl: [&[u8]; 3] = [&arr[0..3], &arr[3..6], &arr[6..9]];
    let iter = arr_sl.iter().arr_chunks::<2>().array_copied();
    let _: (usize, [_; 2]) = iter.collect_arr_zeroed();
}
