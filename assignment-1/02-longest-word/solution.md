# Solution: Longest word slice

## Approach

The task is to find the longest whitespace-separated word in a sentence and return it as an `Option<&str>`.
We achieve this by:
1. Splitting the input string on whitespace using `sentence.split_whitespace()`, which handles any type of Unicode whitespace and filters out empty results.
2. Tracking the longest word found so far using a `longest` variable of type `Option<&str>`.
3. Iterating through each word in the iterator:
   - If `longest` is `None`, we set `longest = Some(word)`.
   - If `longest` is `Some(curr)`, we compare `word.len()` with `curr.len()`. Since we want to return the **first** word in case of a tie, we only update `longest` if the current word's length is strictly greater than the previously recorded longest word's length (`word.len() > curr.len()`).
4. Returning `longest`.

This approach avoids allocating any memory for `String` and borrows directly from `sentence`, fulfilling the constraints.

## Complexity

- **Time Complexity**: $O(N)$ where $N$ is the number of bytes in `sentence`. We iterate through the string once to find whitespace boundaries.
- **Space Complexity**: $O(1)$ extra space beyond the iterator state, as we only store references to parts of the input slice.
