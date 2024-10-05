#![allow(dead_code)]

use std::vec;

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1_iterator = nums1.iter().peekable();
        let mut nums2_iterator = nums2.iter().peekable();
        let mut merged_vector: Vec<i32> = vec![];
        let median_index: f64 = (nums1.len() as f64 + nums2.len() as f64 - 1.0) / 2.0;

        // Iterate only until the vector half point + 1, as we only need the median element(s)
        // while nums1_iterator.peek().is_some() || nums2_iterator.peek().is_some() {
        while merged_vector.len() < median_index.ceil() as usize + 1 {
            match (nums1_iterator.peek(), nums2_iterator.peek()) {
                (Some(n1), Some(n2)) => {
                    if n1 < n2 {
                        merged_vector.push(**n1);
                        nums1_iterator.next();
                    } else {
                        merged_vector.push(**n2);
                        nums2_iterator.next();
                    }
                }

                (Some(n1), None) => {
                    merged_vector.push(**n1);
                    nums1_iterator.next();
                }

                (None, Some(n2)) => {
                    merged_vector.push(**n2);
                    nums2_iterator.next();
                }

                (None, None) => {}
            }
        }

        (merged_vector[median_index.floor() as usize] + merged_vector[median_index.ceil() as usize])
            as f64
            / 2.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scenario1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];

        let expected_result = 2.0;
        let actual_result = Solution::find_median_sorted_arrays(nums1, nums2);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn scenario2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        let expected_result = 2.5;
        let actual_result = Solution::find_median_sorted_arrays(nums1, nums2);

        assert_eq!(expected_result, actual_result);
    }
}
