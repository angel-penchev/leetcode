#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();

        for current_lenght in (1..=len).rev() {
            for current_offset in 0..=(len - current_lenght) {
                let mut letters: HashSet<char> = HashSet::new();
                for letter in s.chars().skip(current_offset).take(current_lenght) {
                    if !letters.insert(letter) {
                        break;
                    }
                }

                if letters.len() == current_lenght {
                    return current_lenght as i32;
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scenario1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn scenario2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn scenario3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn submition_senario1() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1)
    }
}
