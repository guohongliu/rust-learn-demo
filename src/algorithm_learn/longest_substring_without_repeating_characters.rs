use std::collections::{HashMap, HashSet};
use std::ptr;

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s_len = s.len();
        let char_vec: Vec<char> = s.chars().collect();
        let mut short_str = String::from("");

        if s_len == 1 {
            return s_len as i32
        }

        for i in 0..s_len {
            let mut str = String::new();
            for j in i ..s_len {
                if str.contains(char_vec[j]) {
                    if (short_str.len() < str.len()) {
                        short_str = str.clone();
                    }
                    break;
                }

                str.push(char_vec[j]);

                if (short_str.len() < str.len()) {
                    short_str = str.clone();
                }
            }
        }
        short_str.len() as i32
    }

    pub fn length_of_longest_substring_v1(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        let mut start = 0;
        let mut max_start = 0;

        for i in 0..chars.len() {
            let mut seen = std::collections::HashSet::new();
            for j in i..chars.len() {
                if seen.contains(&chars[j]) {
                    break;
                }
                seen.insert(chars[j]);
                if j - i + 1 > max_len { // 如果当前字串最长
                    max_len = j - i + 1;
                    max_start = i;
                }
            }
        }
        chars[max_start..max_start + max_len]
            .iter()
            .collect::<String>().len() as i32
    }

    pub fn length_of_longest_substring_v2(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut seen = HashSet::new();
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..chars.len() {
            // 使用 while 而不是 if：因为可能需要多次移动 left 才能消除重复。
            while seen.contains(&chars[right]) {
                seen.remove(&chars[left]);
                left += 1;
            }
            seen.insert(chars[right]);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }

    pub fn length_of_longest_substring_v3(s: String) -> i32 {
        let mut index = [-1; 128];
        let mut longest = 0;
        let mut start = 0;

        for (i, &b) in s.as_bytes().iter().enumerate() {
            start = start.max(index[b as usize] as usize + 1);
            longest = longest.max(i - start + 1);
            index[b as usize] = i as i32;
        }

        longest as i32
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_hash_map_operations() {
        let str =  " ".to_string();
        // assert_eq!(Solution::length_of_longest_substring(str), 1);
        // assert_eq!(Solution::length_of_longest_substring_v1(str), 1);
        // assert_eq!(Solution::length_of_longest_substring_v2(str), 1);
        assert_eq!(Solution::length_of_longest_substring_v3(str), 1);
    }
}