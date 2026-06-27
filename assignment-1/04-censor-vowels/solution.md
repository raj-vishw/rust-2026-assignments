# Solution: Censor vowels in place

## Approach

The task is to replace every ASCII vowel (`a e i o u`, both cases) in a mutable `String` with `*` in-place, without returning a new String.

Since Rust `String`s are guaranteed to be valid UTF-8, we can mutate the underlying bytes directly as long as we guarantee that we do not violate UTF-8 validity constraints.
- Any ASCII vowel (`a`, `e`, `i`, `o`, `u`, `A`, `E`, `I`, `O`, `U`) has a byte value in the range `0x00` to `0x7F` (representing a single-byte UTF-8 character).
- The replacement character `*` also has a byte value in the range `0x00` to `0x7F` (`0x2A`).
- In UTF-8, multi-byte sequences consist entirely of bytes in the range `0x80` to `0xFF`.
- Therefore, replacing any ASCII byte with another ASCII byte does not affect multi-byte sequences and cannot result in invalid UTF-8.

To achieve this in-place with zero allocations and high performance:
1. We use an `unsafe` block to get a mutable reference to the underlying byte slice using `s.as_bytes_mut()`.
2. We iterate over the bytes and match against the ASCII byte representations of the vowels.
3. If a match is found, we replace it with `b'*'`.

This is completely safe in practice because it preserves the UTF-8 invariants.

## Complexity

- **Time Complexity**: $O(N)$ where $N$ is the number of bytes in `s`.
- **Space Complexity**: $O(1)$ extra space since we modify the bytes in-place.
