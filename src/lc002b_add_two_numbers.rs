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
        let mut l1_node = &l1;
        let mut l2_node = &l2;
        let mut result = Option::None;
        let mut current_result = &mut result;
        let mut carry = 0;

        while l1_node.is_some() || l2_node.is_some() || carry != 0 {
            let mut current_sum = carry;

            if let Some(node) = l1_node {
                current_sum += node.val;
                l1_node = &node.next;
            }

            if let Some(node) = l2_node {
                current_sum += node.val;
                l2_node = &node.next;
            }

            let new_node = Some(Box::new(ListNode::new(current_sum % 10)));

            *current_result = new_node;
            current_result = &mut current_result.as_mut().unwrap().next;
            carry = current_sum / 10;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number_to_list(number: i64) -> Option<Box<ListNode>> {
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

    #[test]
    fn scenario1() {
        let l1 = number_to_list(342);
        let l2 = number_to_list(465);

        let expected_result = number_to_list(807);
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
        let l1 = number_to_list(9_999_999);
        let l2 = number_to_list(9_999);

        let expected_result = number_to_list(10_009_998);
        let actual_result = Solution::add_two_numbers(l1, l2);

        assert_eq!(expected_result, actual_result);
    }
}
