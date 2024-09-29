#![allow(dead_code)]
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut itterated_numbers: HashMap<i32, i32> = HashMap::new();

        for (index, number) in numbers.into_iter().enumerate() {
            if let Some(found) = itterated_numbers.get(&(target - number)) {
                return vec![*found, index as i32];
            }

            itterated_numbers.insert(number, index as i32);
        }

        vec![] // By definition, we should always have a solution and this line should never be executed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let expected_result = vec![0, 1];
        let actual_result = Solution::two_sum(numbers, target);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn scenario2() {
        let numbers = vec![3, 2, 4];
        let target = 6;
        let expected_result = vec![1, 2];
        let actual_result = Solution::two_sum(numbers, target);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn scenario3() {
        let numbers = vec![3, 3];
        let target = 6;
        let expected_result = vec![0, 1];
        let actual_result = Solution::two_sum(numbers, target);
        assert_eq!(actual_result, expected_result);
    }
}
