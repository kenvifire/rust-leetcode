struct Solution {}
pub fn is_palindrome(s: String) -> bool {
    let mut chars:Vec<char> = Vec::new();
    for  c in s.chars() {
        if c.is_alphabetic() {
            chars.push(c.to_ascii_lowercase());
        }
    }

    let mut left = 0;
    let mut right = chars.len() - 1;
    while left <= right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -=1;

    }
    true
}