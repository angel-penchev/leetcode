#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x = x.to_string();
        let x_median = x.len() / 2 - 1;
        //reversed because we will be going to the left
        let mut x_l = x.chars().rev().peekable();
        x_l.nth(x_median);
        let mut x_r = x.chars().peekable();
        x_r.nth(x_median);

        while x_l.peek().is_some()
            && x_r.peek().is_some()
            && x_l.peek().unwrap() == x_r.peek().unwrap()
        {
            println!("x_l: {}, x_r: {}", x_l.peek().unwrap(), x_r.peek().unwrap());
            x_l.next();
            x_r.next();
        }

        x_l.peek().is_none() && x_r.peek().is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn senario_1() {
        let x = 121;
        let expected_result = true;
        let actual_result = Solution::is_palindrome(x);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn senario_2() {
        let x = -121;
        let expected_result = false;
        let actual_result = Solution::is_palindrome(x);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn senario_3() {
        let x = 10;
        let expected_result = false;
        let actual_result = Solution::is_palindrome(x);
        assert_eq!(expected_result, actual_result);
    }
}
