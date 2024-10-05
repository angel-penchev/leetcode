#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result: i32 = 0;
        let is_positive = x >= 0;
        if !is_positive {
            x = -x;
        }

        while x > 0 {
            match result.checked_mul(10) {
                Some(res) => result = res,
                None => return 0,
            }

            match result.checked_add(x % 10) {
                Some(res) => result = res,
                None => return 0,
            }

            x /= 10;
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
    fn scenario1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn scenario2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn scenario3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn personal_scenario1() {
        assert_eq!(Solution::reverse(1_999_999_999), 0);
    }

    #[test]
    fn submittion_scenario1() {
        assert_eq!(Solution::reverse(-2_147_483_412), -2_143_847_412);
    }
}
