struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        
        let mut  i = 0;
        let mut last = 0;

        let bytes = s.as_bytes();
        while i < s.len() {
            while i < s.len() && bytes[i] as char == ' ' {
                i += 1;
            }

            let mut curr = 0;
            while i < s.len() && bytes[i] as char != ' ' {
                curr += 1;
                i += 1;
            }
            if curr > 0 {
                last = curr;
            }

        }
        last
    }
}

mod test {
    use crate::solution_58::Solution;

    #[test]
    pub fn test_basic() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_string()), 6);
        assert_eq!(Solution::length_of_last_word("Today is a nice day".to_string()), 3);

    }
}