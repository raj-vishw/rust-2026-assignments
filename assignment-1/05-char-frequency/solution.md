# Solution: Character frequency, sorted

## Approach

The task is to count the frequency of each distinct character in a given input string and return a list of `(char, count)` pairs sorted in descending order of counts, with alphabetical order breaking ties.

We accomplish this using the following steps:
1. Initialize a `HashMap<char, u32>` to collect and increment the frequency counts of each character as we iterate through `input.chars()`.
2. Convert the `HashMap` into a `Vec<(char, u32)>` using `.into_iter().collect()`.
3. Sort the vector using a custom comparison closure with `.sort_by()`:
   - Primary ordering: Count in descending order (`count2.cmp(count1)`).
   - Secondary ordering: Character in ascending order (`char1.cmp(char2)`).
4. Return the sorted vector.

## Complexity

- **Time Complexity**: $O(N + K \log K)$ where $N$ is the number of characters in the input, and $K$ is the number of unique characters. Populating the hashmap takes $O(N)$, and sorting the unique characters takes $O(K \log K)$.
- **Space Complexity**: $O(K)$ to store the frequency map and vector of unique characters.
