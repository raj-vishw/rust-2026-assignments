# Solution: Group anagrams

## Approach

The objective is to group words that are anagrams of each other.
- Comparisons must be case-insensitive.
- Output words must preserve their original casing.
- Order of words within each group must follow their original order of appearance in the input.
- All words consist of ASCII characters only.

We implement this using a combination of a hash map and a key tracking vector to preserve insertion order:
1. Initialize a `HashMap<String, Vec<String>>` to hold the grouped anagrams, mapping a sorted-letter signature to the list of matching words.
2. Initialize a `Vec<String>` called `group_keys` to record the order in which new anagram signatures are encountered.
3. For each word in the input:
   - Convert the word to lowercase and extract its bytes: `word.to_ascii_lowercase().into_bytes()`.
   - Sort the bytes in-place: `signature_bytes.sort_unstable()`.
   - Construct the signature string: `String::from_utf8(signature_bytes).unwrap()`.
   - Look up the signature in the hash map. If it's a new signature (vacant entry), append it to `group_keys`.
   - Push the original word (preserving casing) to the vector associated with that signature.
4. Construct the final output by iterating through `group_keys` and pulling the corresponding group from the map. This ensures group order matches the first appearance of any word in the group, and avoids non-deterministic group ordering.

## Complexity

- **Time Complexity**: $O(N \cdot L \log L)$ where $N$ is the number of words and $L$ is the maximum length of a word. Sorting the letters of each word of length $L$ takes $O(L \log L)$ time. Inserting and retrieving from the hash map takes $O(L)$ on average due to hashing.
- **Space Complexity**: $O(N \cdot L)$ to store the signatures and groups in the map and the final output list.
