# 4. Censor vowels in place

Replace every ASCII vowel (`a e i o u`, both cases) in `s` with `*`. Mutate `s` directly: **do not** return a new `String`.

```rust
pub fn censor_vowels(s: &mut String);
```

### Examples

- `"Hello, World!"` → `"H*ll*, W*rld!"`
- `"AEIOU"` → `"*****"`
- `""` → `""`

### Hint

A `String` of ASCII is easiest to mutate via `unsafe { s.as_bytes_mut() }` *or* by rebuilding through `chars()`. Pick whichever you can defend; both are fine here.

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
