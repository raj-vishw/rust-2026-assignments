# Solution: Inventory: ownership vs borrowing

## Approach

The task involves managing inventory pairs represented as `(String, u32)` tuples. We need to implement two functions:
1. `restock`: Merges two lists of inventory items. It consumes both input vectors (`Vec<(String, u32)>`) and returns a new merged `Vec<(String, u32)>` where duplicate names have their quantities summed up.
2. `summary`: Summarizes the inventory list by returning a formatted string without consuming the list (borrowing via `&[(String, u32)]`).

### Implementation Details:
- **`restock`**: 
  - To achieve optimal $O(M + N)$ time complexity where $M$ and $N$ are the sizes of the two lists, we insert the elements into a `HashMap<String, u32>`.
  - The hash map is initialized using `HashMap::with_capacity(inventory.len() + more.len())` to minimize allocations/re-hashes.
  - Since `restock` takes ownership of the inputs, we can directly consume the vectors and insert their items into the map.
  - Finally, we convert the map back to a vector via `map.into_iter().collect()`.
- **`summary`**:
  - The function only borrows the slice. We calculate `items` as `inventory.len()`.
  - We calculate total `units` by mapping over the quantities and summing them: `inventory.iter().map(|(_, qty)| qty).sum()`.
  - We format the result as `"{items} items, {units} units"` and return it.

This satisfies the compilation requirement where `summary` is called first (borrowing `inv`) and `restock` is called second (consuming `inv`).

## Complexity

- **`restock`**:
  - **Time Complexity**: $O(M + N)$ where $M$ is the size of `inventory` and $N$ is the size of `more`.
  - **Space Complexity**: $O(M + N)$ to store the merged items in the hash map.
- **`summary`**:
  - **Time Complexity**: $O(K)$ where $K$ is the size of the slice, to sum the elements.
  - **Space Complexity**: $O(1)$ extra space beyond allocating the output summary string.
