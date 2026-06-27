pub fn censor_vowels(s: &mut String) {
    // SAFETY: We are only mutating ASCII characters ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U')
    // into another ASCII character ('*'). Since all ASCII characters fit in a single byte in UTF-8
    // (values < 0x80), and UTF-8 multi-byte sequence bytes always have the high bit set (>= 0x80),
    // changing ASCII bytes to other ASCII bytes cannot violate UTF-8 validity.
    unsafe {
        let bytes = s.as_bytes_mut();
        for b in bytes {
            match b {
                b'a' | b'e' | b'i' | b'o' | b'u' |
                b'A' | b'E' | b'I' | b'O' | b'U' => {
                    *b = b'*';
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> String {
        let mut s = String::from(input);
        censor_vowels(&mut s);
        s
    }

    #[test]
    fn example_hello_world() {
        assert_eq!(run("Hello, World!"), "H*ll*, W*rld!");
    }

    #[test]
    fn all_uppercase_vowels() {
        assert_eq!(run("AEIOU"), "*****");
    }

    #[test]
    fn empty_input() {
        assert_eq!(run(""), "");
    }

    #[test]
    fn no_vowels() {
        assert_eq!(run("bcdfg"), "bcdfg");
    }

    #[test]
    fn all_lowercase_vowels() {
        assert_eq!(run("aeiou"), "*****");
    }

    #[test]
    fn mixed_case() {
        assert_eq!(run("AaEeIi"), "******");
    }

    #[test]
    fn digits_and_letters() {
        assert_eq!(run("h3ll0 wOrld"), "h3ll0 w*rld");
    }

    #[test]
    fn punctuation_only() {
        assert_eq!(run("!@#$%"), "!@#$%");
    }
}
