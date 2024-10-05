#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut result = String::from("");

        for index in 0..s.len() {
            // Check max palindrome with current_letter as a center
            let mut l = index;
            let mut r = index;

            while r < s.len() && s.get(l..l + 1).unwrap() == s.get(r..r + 1).unwrap() {
                if r - l + 1 > result.len() {
                    result = s[l..r + 1].to_string();
                }

                if l == 0 {
                    break;
                }

                l -= 1;
                r += 1;
            }

            // Check max palindrome with current_letter and adjacent letter as a center
            l = index;
            r = index + 1;

            while r < s.len() && s.get(l..l + 1).unwrap() == s.get(r..r + 1).unwrap() {
                if r - l + 1 > result.len() {
                    result = s[l..r + 1].to_string();
                }

                if l == 0 {
                    break;
                }

                l -= 1;
                r += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scenario1() {
        let s = String::from("babad");
        let expected_result = String::from("bab");
        let actual_result = Solution::longest_palindrome(s);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn scenario2() {
        let s = String::from("cbbd");
        let expected_result = String::from("bb");
        let actual_result = Solution::longest_palindrome(s);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn submition_senario1() {
        let s = String::from("a");
        let expected_result = String::from("a");
        let actual_result = Solution::longest_palindrome(s);
        assert_eq!(expected_result, actual_result);
    }
}
