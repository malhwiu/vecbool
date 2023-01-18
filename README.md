# vecbool

A simple implementation of a bit vector built on top of `Vec<u8>`.

The `vecbool::VecBool` was implemented to replace a `Vec<bool>` while retaining similar performance and reducing memory usage - since a `u8` can pack 8 `bool`, while a `Vec<bool>` uses one byte to store one `bool`.

**NOTE**: This is mostly a toy project used for learning purposes. If you want a more robust alternative check the crate [`bitvec`](https://docs.rs/bitvec/latest/bitvec/index.html).
If you found something which can be improved, feel free to create an issue on github. Any feedback is more than welcomed!

# Examples

```rust
use vecbool::vecbool::VecBool;

let mut vecbool = VecBool::new();

assert_eq!(vecbool.get(0), None);

vecbool.push(true);
vecbool.push(false);
assert_eq!(vecbool.get(0), Some(true));
assert_eq!(vecbool.get(1), Some(false));

let vec: Vec<_> = vecbool.iter().collect();
assert_eq!(vec, vec![true, false]);

assert_eq!(vecbool.pop(), Some(false));
assert_eq!(vecbool.pop(), Some(true));
assert_eq!(vecbool.pop(), None);
```

# Benchmarks

Benchmarks were made comparing `BoolVec`, `Vec<bool>` and [`BitVec`](https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html). The benchmarks were done using rust's default benchmark tools (available on nightly builds), which I'm not used and thus the results can easily be wrong. Anyhow, the results obtained are the followings:

Iterating elements in vectors with `n` elements - `VecBool` much better performance for `1_000_000` elements, probably due
to some kind of autovectorization shenanigans:

```
test bitvec::iter_10_elements         ... bench:          10 ns/iter (+/- 0)
test vec::iter_10_elements            ... bench:           2 ns/iter (+/- 0)
test vecbool::iter_10_elements        ... bench:           6 ns/iter (+/- 0)

test bitvec::iter_1000_elements       ... bench:         739 ns/iter (+/- 753)
test vec::iter_1000_elements          ... bench:         267 ns/iter (+/- 6)
test vecbool::iter_1000_elements      ... bench:         274 ns/iter (+/- 313)

test bitvec::iter_1_000_000_elements  ... bench:     660,920 ns/iter (+/- 11,827)
test vec::iter_1_000_000_elements     ... bench:     306,850 ns/iter (+/- 405,266)
test vecbool::iter_1_000_000_elements ... bench:     186,838 ns/iter (+/- 1,389)
```

Accessing `n` random indexes in vectors with `n` elements - comparable performance:

```
test bitvec::get_10_elements          ... bench:           3 ns/iter (+/- 0)
test vec::get_10_elements             ... bench:           4 ns/iter (+/- 1)
test vecbool::get_10_elements         ... bench:           7 ns/iter (+/- 2)

test bitvec::get_1000_elements        ... bench:         295 ns/iter (+/- 242)
test vec::get_1000_elements           ... bench:         391 ns/iter (+/- 3)
test vecbool::get_1000_elements       ... bench:         750 ns/iter (+/- 859)

test bitvec::get_1_000_000_elements   ... bench:     603,804 ns/iter (+/- 26,881)
test vec::get_1_000_000_elements      ... bench:     613,401 ns/iter (+/- 471,768)
test vecbool::get_1_000_000_elements  ... bench:     663,668 ns/iter (+/- 209,096)
```

Pushing `n` elements to empty vectors - comparable perfomance:

```
test bitvec::push_10_elements         ... bench:          95 ns/iter (+/- 0)
test vec::push_10_elements            ... bench:         165 ns/iter (+/- 6)
test vecbool::push_10_elements        ... bench:          94 ns/iter (+/- 88)

test bitvec::push_1000_elements       ... bench:       5,520 ns/iter (+/- 6,270)
test vec::push_1000_elements          ... bench:       3,014 ns/iter (+/- 3,864)
test vecbool::push_1000_elements      ... bench:       2,985 ns/iter (+/- 5,452)

test bitvec::push_1_000_000_elements  ... bench:   3,276,390 ns/iter (+/- 5,365,655)
test vec::push_1_000_000_elements     ... bench:   2,661,370 ns/iter (+/- 4,383,416)
test vecbool::push_1_000_000_elements ... bench:   2,486,280 ns/iter (+/- 1,126,453)
```

The benchmark code is available [here](./benchmarks/src).
