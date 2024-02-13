use iterextd::IterExtd;

fn main() {
    let arr = [10u8, 20u8, 40u8, 50u8];
    let iter = arr.iter();
    let arr_coll: (usize, [_; 10]) = iter.collect_arr_zeroed();
}
