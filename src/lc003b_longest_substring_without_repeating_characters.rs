#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut letter_latest_occurances: HashMap<char, usize> = HashMap::new();
        let mut max_lenght: usize = 0;
        let mut current_length: usize = 0;

        for (index, letter) in s.chars().enumerate() {
            match letter_latest_occurances.get(&letter) {
                Some(previous_index) => {
                    if *previous_index < index - current_length {
                        current_length += 1;
                    } else {
                        current_length = index - previous_index;
                    }
                    max_lenght = usize::max(max_lenght, current_length);
                }
                None => {
                    current_length += 1;
                    max_lenght = usize::max(max_lenght, current_length);
                }
            }

            letter_latest_occurances.insert(letter, index);
        }

        max_lenght as i32
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

    #[test]
    fn submition_senario2() {
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2)
    }

    #[test]
    fn submition_senario3() {
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2)
    }

    #[test]
    fn submition_senario4() {
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3)
    }
}
