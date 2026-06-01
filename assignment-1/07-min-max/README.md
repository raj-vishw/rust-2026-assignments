# 7. `min_max`: no iterator helpers

Return `(min, max)` of a slice of `i32`. **Do not** call `.iter().min()`, `.max()`, `.minmax()`, or any iterator adapter: use a manual loop. Return `None` for an empty slice.

```rust
pub fn min_max(xs: &[i32]) -> Option<(i32, i32)>;
```

### Examples

- `&[3, 1, 4, 1, 5, 9, 2, 6]` → `Some((1, 9))`
- `&[]` → `None`
- `&[7]` → `Some((7, 7))`

### Constraint

A single `for` loop over `xs` updating two locals is the whole point. **No iterator adapters.**

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
