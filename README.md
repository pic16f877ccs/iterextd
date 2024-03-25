# iterextd

Adapters that extend `Iterator` functionality.

## Example Usages

### Variable step

```rust
use iterextd::IterExtd;

let logic_fn = |s: &mut usize| { if *s == 0 { *s = 1; 1 } else { *s += 1; *s } };
let iter = (0..18).step_by_fn(logic_fn);
let vec = iter.collect::<Vec<_>>();
assert_eq!(vec, vec![0, 2, 5, 9, 14]);
```

### Collect a zeroed array

```rust
use iterextd::IterExtd;

let arr = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let iter = arr.iter().filter(|elem| *elem % 2 == 0).copied();
let arr: (usize, [u8; 10]) = iter.collect_arr_zeroed();
assert_eq!(arr, (5, [2, 4, 6, 8, 10, 0, 0, 0, 0, 0]));
```

### Collect windows from arrays.

```rust
use iterextd::IterExtd;

let arr = [0, 1, 2, 3, 4, 5, 6, 7];
let iter = arr.into_iter();
let iter = iter.clone().map_iters(iter.previous(1).skip(2), |self_iter, arg_iter| {
    let (pre_elem, elem) = arg_iter.next()?;
    Some([self_iter.next()?, pre_elem, elem])
});
let vec = iter.collect::<Vec<_>>();
assert_eq!(vec, vec![[0, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5], [4, 5, 6], [5 , 6, 7]]);
```

## Features

- `iterextd` provides several adapters to extend the functionality of iterators in Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iterextd = "0.3.0"
```

## License

This project is licensed under the MIT License.
