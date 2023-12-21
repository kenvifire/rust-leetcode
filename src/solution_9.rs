struct Solution();
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let str = x.to_string();
            let bytes = str.as_bytes();
            let len = str.len();
            let mut start = 0;
            let mut end = len - 1;
            while start < end {
                if bytes[start] != bytes[end] {
                    return false;
                }
                start += 1;
                end -=1;
            }
            return true;
        }


    }
}


#[cfg(test)]
mod test {
    use crate::solution_9::Solution;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(1), true);
        assert_eq!(Solution::is_palindrome(21), false);
    }
}