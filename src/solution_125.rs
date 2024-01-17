use std::panic::AssertUnwindSafe;

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let mut chars: Vec<char> = Vec::new();
        for c in s.chars() {
            if c.is_alphanumeric() {
                chars.push(c.to_ascii_lowercase());
            }
        }

        let mut left: i32 = 0;
        let mut right:i32 = chars.len() as i32 - 1;
        while left <= right {
            if chars[left as usize] != chars[right as usize] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[test]
pub fn test_basic() {
    // assert_eq!(Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")), true);
    // assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
    assert_eq!(Solution::is_palindrome(String::from("0P")), false);
}