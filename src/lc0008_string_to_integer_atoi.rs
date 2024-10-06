#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        let mut result: i32 = 0;
        let mut is_positive = true;
        let mut s_chars = s.chars();

        match s_chars.next() {
            Some('-') => is_positive = false,
            Some('+') => is_positive = true,
            Some(character) => {
                if character.is_ascii_digit() {
                    result += character.to_digit(10).unwrap() as i32;
                } else {
                    return 0;
                }
            }
            None => return 0,
        }

        for character in s_chars {
            if character.is_ascii_digit() {
                match result.checked_mul(10) {
                    Some(res) => result = res,
                    None => return if is_positive { i32::MAX } else { i32::MIN },
                }
                match result.checked_add(character.to_digit(10).unwrap() as i32) {
                    Some(res) => result = res,
                    None => return if is_positive { i32::MAX } else { i32::MIN },
                }
            } else {
                break;
            }
        }

        if is_positive {
            result
        } else {
            -result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scenario_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn scenario_2() {
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
    }

    #[test]
    fn scenario_3() {
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    }

    #[test]
    fn scenario_4() {
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    }

    #[test]
    fn scenario_5() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}
