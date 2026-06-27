# Solution: Caesar cipher with a `const` alphabet

## Approach

The task is to shift each alphabetic character forward/backward by a certain offset within the alphabet specified by the module-level constant `ALPHABET`. We must preserve the case (uppercase/lowercase) and leave other non-alphabetic characters untouched. We are not allowed to hardcode `26`.

We implement this as follows:
1. Extract the alphabet size dynamically: `let len = ALPHABET.len() as i32;`.
2. Normalize the shift value to be within the bounds `[0, len)` using the formula `((shift % len) + len) % len`. This ensures that negative shifts and shifts larger than 26 are correctly wrapped.
3. Map over each character in the input string:
   - Check if the lowercase version of the character exists in the `ALPHABET`.
   - If it does, we compute the shifted index `new_idx = ((idx as i32 + shift) % len) as usize`.
   - We retrieve the shifted lowercase character from the `ALPHABET` bytes: `ALPHABET.as_bytes()[new_idx] as char`.
   - If the original character was uppercase, we convert the result back to uppercase.
   - Otherwise, we return the character as-is.
4. Collect the mapped characters into a new `String`.

## Complexity

- **Time Complexity**: $O(N)$ where $N$ is the number of characters in the input. For each character, finding its index in `ALPHABET` takes $O(1)$ time because `ALPHABET` size is constant (26).
- **Space Complexity**: $O(N)$ for allocating the returned `String`.
