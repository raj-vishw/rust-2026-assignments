# Solution: Split & double: disjoint mutable borrows

## Approach

The task requires splitting a mutable vector of integers `xs` into two disjoint slices at index `mid`, doubling all elements in-place, and returning the two slices as a tuple `(&mut [i32], &mut [i32])`.

Under Rust's ownership model, we cannot borrow multiple overlapping or potentially overlapping mutable references of the same structure. However, the standard library provides `split_at_mut` specifically for this pattern:
1. We call `xs.split_at_mut(mid)`. This returns two disjoint mutable slices: `left` representing `&mut xs[0..mid]` and `right` representing `&mut xs[mid..]`.
2. `split_at_mut` naturally panics if `mid > xs.len()`, satisfying the error/panic constraint.
3. We then iterate over each slice (`left` and `right`) using `iter_mut()` to double every element in place.
4. Finally, we return the tuple `(left, right)`.

Because the borrow checker knows that `split_at_mut` returns two non-overlapping slices, it safely permits this disjoint mutable borrow.

## Complexity

- **Time Complexity**: $O(N)$ where $N$ is the number of elements in the vector `xs`, since we visit and double each element exactly once.
- **Space Complexity**: $O(1)$ extra space beyond the return slice references.
