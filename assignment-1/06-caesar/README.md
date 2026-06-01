# 6. Caesar cipher with a `const` alphabet

Shift each ASCII letter forward by `shift` positions, wrapping inside the 26-letter alphabet. Preserve case. Leave digits, punctuation, and whitespace untouched. `shift` may be negative or larger than 26.

Declare the alphabet as a module-level `const`:

```rust
pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String;
```

### Examples

- `caesar("Hello, World!", 3)` → `"Khoor, Zruog!"`
- `caesar("abc", -1)` → `"zab"`
- `caesar("xyz", 27)` → `"yza"`

### Constraint

You must use `ALPHABET` (or `ALPHABET.len()`) somewhere in your implementation: **don't hard-code `26`**.

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
