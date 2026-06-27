pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String {
    let len = ALPHABET.len() as i32;
    let shift = ((shift % len) + len) % len;

    input.chars().map(|c| {
        let is_upper = c.is_ascii_uppercase();
        let lower_c = c.to_ascii_lowercase();
        if let Some(idx) = ALPHABET.find(lower_c) {
            let new_idx = ((idx as i32 + shift) % len) as usize;
            let shifted_char = ALPHABET.as_bytes()[new_idx] as char;
            if is_upper {
                shifted_char.to_ascii_uppercase()
            } else {
                shifted_char
            }
        } else {
            c
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
