use super::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let mut result = &s[0..1];

        for i in 0..(s.len() - 1) {
            // let max_length = i.min(s.len() - 1 - i);
            if result.len() / 2 > s.len() - i {
                break;
            }
            let mut left = i;
            let mut right = i;

            while left > 0
                && right < s.len() - 1
                && s.chars().nth(left - 1) == s.chars().nth(right + 1)
            {
                left -= 1;
                right += 1;
            }

            if right - left + 1 > result.len() {
                result = &s[left..=right];
            }

            left = i;
            right = i + 1;

            if s.chars().nth(left) != s.chars().nth(right) {
                continue;
            }

            while left > 0
                && right < s.len() - 1
                && s.chars().nth(left - 1) == s.chars().nth(right + 1)
            {
                left -= 1;
                right += 1;
            }

            if right - left + 1 > result.len() {
                result = &s[left..=right];
            }
        }

        return result.to_string();
    }
}
