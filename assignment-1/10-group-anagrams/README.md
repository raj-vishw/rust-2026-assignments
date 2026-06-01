# 10. Group anagrams

Group words that are anagrams of each other. Return a `Vec` of groups; each group is a `Vec<String>` of the words from the input that share a sorted-letter signature. Group order is unspecified, but within each group keep the **input order**. Compare case-insensitively; output words preserve their original casing. Words are ASCII letters only.

```rust
pub fn group_anagrams(words: &[String]) -> Vec<Vec<String>>;
```

### Examples

- Input `["Eat", "tea", "tan", "ate", "Nat", "bat"]`
  - One acceptable output: `[["Eat", "tea", "ate"], ["tan", "Nat"], ["bat"]]`
- Input `[]` → `[]`

### Hint

Build the signature by sorting the lowercased characters of each word; group by signature in a `HashMap<String, Vec<String>>`, then collect.

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
