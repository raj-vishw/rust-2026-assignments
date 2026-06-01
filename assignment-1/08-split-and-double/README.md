# 8. Split & double: disjoint mutable borrows

Split `xs` into a left half and a right half at index `mid`, double every element in each half **in place**, and return the two `&mut [i32]` slices.

```rust
pub fn split_and_double(xs: &mut Vec<i32>, mid: usize) -> (&mut [i32], &mut [i32]);
```

### Examples

- After `split_and_double(&mut vec![1, 2, 3, 4], 2)`:
  - `xs == [2, 4, 6, 8]`
  - returned slices are `[2, 4]` and `[6, 8]`.

### Constraint

- Panic if `mid > xs.len()` (let the standard library do it for you).
- The two returned slices must be **disjoint mutable borrows** of `xs`.

### Hint

`split_at_mut` is what the borrow checker is waiting for.

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
