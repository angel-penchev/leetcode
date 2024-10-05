#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let lenght = s.len();
        let mut result = String::from("");

        // First row
        // Takes num_rows - 1 to get to the bottom and num_rows to get back
        for letter in s
            .chars()
            .step_by(if num_rows == 1 { 1 } else { (num_rows - 1) * 2 })
        {
            result.push(letter);
        }

        if num_rows == 1 {
            return result;
        }

        // In-between rows
        for row in 1..num_rows - 1 {
            // We start from the first letter in the row and then we step to the next letter.
            // The next letter is always after we go the end of the row and then the same distance
            // back.
            // For example, for PAYPALISHIRING and 5 rows:
            // P     H           -> first row
            // A   S I           -> step by 6 first and then by 2
            // Y  I  R           -> step by 4 first and then by 4
            // P L   I G         -> step by 2 first and then by 6
            // A     N           -> last row

            let mut current_letter_index = row;
            let mut step_by = (num_rows - 1 - row) * 2;

            while current_letter_index < lenght {
                let current_letter = s
                    .get(current_letter_index..current_letter_index + 1)
                    .unwrap();
                result.push_str(current_letter);

                current_letter_index += step_by;
                step_by = (num_rows - 1) * 2 - step_by;
            }
        }

        // Last row
        // Skip to the last row, then it takes num_rows to get to the top and num_rows - 1 to get back
        for letter in s.chars().skip(num_rows - 1).step_by((num_rows - 1) * 2) {
            result.push(letter);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn scenario2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }

    #[test]
    fn submittion_scenario1() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }
}
