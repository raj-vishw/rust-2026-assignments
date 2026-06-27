# Solution notes: Run-length encode

## Approach

The function `run_length_encode` compresses consecutive runs of equal characters in a string into `(char, count)` pairs. 
Our approach is:
1. Initialize an empty vector `result`.
2. Extract the character iterator from the input string using `input.chars()`.
3. Retrieve the first character if it exists. If the input is empty, return the empty vector.
4. Track the `current_char` and its run `count` (initialized to 1).
5. Loop through the remaining characters. If a character matches `current_char`, increment `count`. Otherwise, push `(current_char, count)` to `result`, update `current_char` to the new character, and reset `count` to 1.
6. After the loop, push the final run `(current_char, count)` to the `result` vector.

## Edge cases handled

- **Empty input string**: Correctly returns an empty vector without panicking.
- **Single character string**: Correctly returns a vector with a single tuple containing that character and a count of 1.
- **Alternating characters/runs**: Handled by properly resetting `count` to 1 and updating `current_char` upon encountering a mismatch.
- **Whitespace characters**: Handled correctly since `input.chars()` treats whitespaces as characters.

## Anything special

The solution runs in $O(N)$ time complexity and uses $O(1)$ extra memory space (excluding the output vector), which is optimal.
