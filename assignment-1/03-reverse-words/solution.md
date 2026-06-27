# Solution: Reverse the word order

## Approach

The requirement is to return a new `String` with the words of a sentence reversed in order, collapsing multiple whitespaces into a single space, and trimming any leading/trailing spaces.

We use the following steps:
1. Call `sentence.split_whitespace()`, which automatically handles trimming external spaces and collapsing multiple consecutive whitespace characters (including tabs and newlines).
2. Call `.rev()` on the iterator to reverse the order of the word slices.
3. Call `.collect::<Vec<&str>>()` to collect the reversed word slices into a temporary vector.
4. Call `.join(" ")` on the vector, which joins the elements using a single space.

This correctly returns an empty string for empty inputs or inputs with only whitespaces.

## Complexity

- **Time Complexity**: $O(N)$ where $N$ is the length of `sentence`, to iterate and tokenize the string, and construct the final String.
- **Space Complexity**: $O(N)$ to hold the temporary `Vec<&str>` of words and build the resulting `String`.
