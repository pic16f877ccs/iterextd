use iterextd::IterExtd;

fn main() {
    let mut arr = [10u8, 11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222];
    let iter = arr.iter().arr_chunks::<5>();
    let _: (usize, [_; 2]) = iter.collect_arr_zeroed();
}
