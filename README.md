# iterextd

Give Rust iterators a few more useful moves. `iterextd` adds adapters for extending numeric sequences, finding gaps in integer data, and selecting recurring batches.

## Examples

### Extend a numeric sequence

Continue a series using its last observed step:

```rust
use iterextd::IterExtd;

let forecast: Vec<_> = [2, 5, 6, 9, 13]
    .into_iter()
    .extrapolate()
    .take(8)
    .collect();
assert_eq!(forecast, [2, 5, 6, 9, 13, 17, 21, 25]);
```

### Find missing integers

Find gaps in IDs without sorting or removing duplicates first:

```rust
use iterextd::IterExtd;

let ids = [8, 3, 5, 3, 6];
let missing: Vec<_> = ids.iter().missing_integers().collect();
assert_eq!(missing, [4, 7]);
```

### Select recurring batches

Take three items, skip two, and repeat:

```rust
use iterextd::IterExtd;

let selected: Vec<_> = (1..=10).take_skip_cyclic(3, 2).collect();
assert_eq!(selected, [1, 2, 3, 6, 7, 8]);
```

## Usage

```toml
[dependencies]
iterextd = "0.9.0"
```

## License

MIT
