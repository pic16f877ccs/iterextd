# iterextd

Give Rust iterators a few more useful moves. `iterextd` adds adapters for transforming values, extending sequences, finding gaps, and generating points.

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

### Scale values to a range

Map input values onto a new range:

```rust
use iterextd::Scaler;

let scaled: Vec<_> = (0..=5u8).scaling::<u16>(..70u8).collect();
assert_eq!(scaled, [0, 14, 28, 42, 56, 70]);
```

### Generate circle points

Create integer coordinates around a circle with Bresenham's algorithm:

```rust
use iterextd::CircleBresenhamSeq;

let points: Vec<_> = CircleBresenhamSeq::<i16>::new(1_u8).collect();
assert_eq!(points, [(0, -1), (1, 0), (0, 1), (-1, 0)]);
```

## Usage

```toml
[dependencies]
iterextd = "0.9.0"
```

## License

MIT
