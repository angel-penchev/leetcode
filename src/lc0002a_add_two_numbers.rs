#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut number1: i64 = 0;
        let mut number2: i64 = 0;

        let mut current_digit = &l1;
        let mut current_digit_i = 0;
        while current_digit.is_some() {
            number1 +=
                current_digit.as_ref().unwrap().val as i64 * i32::pow(10, current_digit_i) as i64;
            current_digit = &(current_digit.as_ref().unwrap().next);
            current_digit_i += 1;
        }

        current_digit = &l2;
        current_digit_i = 0;
        while current_digit.is_some() {
            number2 +=
                current_digit.as_ref().unwrap().val as i64 * i32::pow(10, current_digit_i) as i64;
            current_digit = &(current_digit.as_ref().unwrap().next);
            current_digit_i += 1;
        }

        Solution::number_to_list(number1 + number2)
    }

    pub fn number_to_list(number: i64) -> Option<Box<ListNode>> {
        let mut mutable_number = number;
        let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: (mutable_number % 10) as i32,
            next: None,
        }));

        mutable_number /= 10;
        let mut current_result_node = &mut result;

        while mutable_number != 0 {
            let next_node = Some(Box::new(ListNode {
                val: (mutable_number % 10) as i32,
                next: None,
            }));
            current_result_node.as_mut().unwrap().next = next_node;

            mutable_number /= 10;
            current_result_node = &mut current_result_node.as_mut().unwrap().next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario1() {
        let l1 = Solution::number_to_list(342);
        let l2 = Solution::number_to_list(465);

        let expected_result = Solution::number_to_list(807);
        let actual_result = Solution::add_two_numbers(l1, l2);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn scenario2() {
        let l1 = Some(Box::new(ListNode { val: 0, next: None }));
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));

        let expected_result = Some(Box::new(ListNode { val: 0, next: None }));
        let actual_result = Solution::add_two_numbers(l1, l2);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn scenario3() {
        let l1 = Solution::number_to_list(9_999_999);
        let l2 = Solution::number_to_list(9_999);

        let expected_result = Solution::number_to_list(10_009_998);
        let actual_result = Solution::add_two_numbers(l1, l2);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn submition_senario1() {
        let l1 = Solution::number_to_list(9999999991);
        let l2 = Solution::number_to_list(9);

        let expected_result = Solution::number_to_list(10000000000);
        let actual_result = Solution::add_two_numbers(l1, l2);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn submition_senario2() {
        let l1 = Solution::number_to_list(1111111111);
        let l2 = Solution::number_to_list(1111111111);

        let expected_result = Solution::number_to_list(2222222222);
        let actual_result = Solution::add_two_numbers(l1, l2);
        assert_eq!(expected_result, actual_result);
    }
}
