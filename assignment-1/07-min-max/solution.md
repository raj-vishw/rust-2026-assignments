# Solution: `min_max` without iterator helpers

## Approach

The requirement is to find the minimum and maximum values of a slice of `i32` values, returning `None` if the slice is empty. We are explicitly forbidden from using iterator adapters (like `.iter().min()`, `.max()`, etc.) and must use a manual loop instead.

Our approach is:
1. Check if the slice `xs` is empty. If it is, return `None`.
2. Initialize two mutable variables, `min` and `max`, with the first element of the slice (`xs[0]`).
3. Iterate over the remaining elements in the slice (`&xs[1..]`) using a `for` loop:
   - If an element is smaller than the current `min`, we update `min`.
   - If an element is larger than the current `max`, we update `max`.
4. Return `Some((min, max))`.

This manual implementation is highly optimal, performs a single pass over the elements, and performs exactly the requested behavior.

## Complexity

- **Time Complexity**: $O(N)$ where $N$ is the length of `xs`, as we inspect each element exactly once.
- **Space Complexity**: $O(1)$ extra space.
