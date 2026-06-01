# 1. Run-length encode

Compress consecutive runs of equal characters into `(char, count)` pairs, preserving original order.

```rust
pub fn run_length_encode(input: &str) -> Vec<(char, u32)>;
```

### Examples

- `"aaabbc"` → `[('a', 3), ('b', 2), ('c', 1)]`
- `""` → `[]`
- `"x"` → `[('x', 1)]`

### Hint

Iterate `input.chars()`; track the current run as you go.

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
