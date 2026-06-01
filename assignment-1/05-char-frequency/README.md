# 5. Character frequency, sorted

Return each distinct character with its count, sorted by **descending count**, ties broken by **ascending character**.

```rust
pub fn char_frequency(input: &str) -> Vec<(char, u32)>;
```

### Examples

- `"mississippi"` → `[('i', 4), ('s', 4), ('p', 2), ('m', 1)]`
- `""` → `[]`
- `"abcabc"` → `[('a', 2), ('b', 2), ('c', 2)]`

### Hint

`HashMap<char, u32>`, then collect to a `Vec` and `sort_by`.

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
