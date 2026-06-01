# 2. Longest word slice

Return a **borrowed slice** of the longest whitespace-separated word. On ties, return the first one. On an empty / whitespace-only input, return `None`.

```rust
pub fn longest_word(sentence: &str) -> Option<&str>;
```

### Examples

- `"the quick brown fox"` → `Some("quick")`
- `"   "` → `None`
- `"a bb ccc dd"` → `Some("ccc")`

### Constraint

The returned `&str` must borrow from `sentence`: **do not allocate a `String`**.

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
