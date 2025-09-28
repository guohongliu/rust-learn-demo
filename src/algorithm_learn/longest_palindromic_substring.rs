struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let chars_len = s.len() as i32;

        let mut result = String::new();
        for i in 1..chars_len {
            let mut left:i32 = i as i32;
            let mut right:i32 = i as i32;
            loop {
                left -= 1;
                right += 1;

                if (left < 0 || right == chars_len) {
                    break;
                }

                if chars[left] == chars[right] {
                    if result.len() < (right - left) {
                        result = (&s[left..=right]).parse().unwrap();
                    }
                    break;
                }
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::longest_palindrome("babad".parse().unwrap()), "bab");
    }
}