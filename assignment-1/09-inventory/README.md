# 9. Inventory: ownership vs borrowing

Each inventory item is a pair `(name, qty)` where `name: String` and `qty: u32`. (We will switch to a `struct` once that module is covered.)

Implement two free functions:

```rust
/// Consume both lists and produce a single merged inventory. Items with the
/// same `name` are combined into one entry; quantities add. Order of items
/// in the result is unspecified.
pub fn restock(
    inventory: Vec<(String, u32)>,
    more: Vec<(String, u32)>,
) -> Vec<(String, u32)>;

/// Borrow the inventory and produce a one-line summary like
/// `"3 items, 17 units"`. Do not consume the input.
pub fn summary(inventory: &[(String, u32)]) -> String;
```

### Constraint

Calling `summary(&inv)` and then `restock(inv, more)` **in that order** must compile. (i.e. `summary` only borrows; `restock` consumes.)

### Your write-up

See [`solution.md`](./solution.md) for the approach you took and anything special worth noting.
