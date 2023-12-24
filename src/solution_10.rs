struct Solution();

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        Solution::is_matched(s.as_bytes(), 0, p.as_bytes(), 0)
    }

    pub fn is_matched(s: &[u8], s_pos: usize, p: &[u8], p_pos: usize) -> bool {
        if s_pos == s.len() && p_pos == p.len() {
            return true
        }
        if s_pos >= s.len() || p_pos >= p.len() {
            return false
        }

        match p[p_pos] as char {
            '*' => Solution::is_matched(s, s_pos + 1, p, p_pos ) || Solution::is_matched(s, s_pos + 1, p, p_pos + 1),
            '.' => Solution::is_matched(s, s_pos + 1, p, p_pos + 1),
            _ => {
                if p[p_pos] == s[s_pos] {
                    Solution::is_matched(s, s_pos + 1, p, p_pos + 1)
                } else {
                    false
                }
            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(Solution::is_match(String::from(""), String::from("")), true);
        assert_eq!(Solution::is_match(String::from("abc"), String::from("abc")), true);
        assert_eq!(Solution::is_match(String::from("abc"), String::from("ab.")), true);
        assert_eq!(Solution::is_match(String::from("abc"), String::from("ab*")), true);
        assert_eq!(Solution::is_match(String::from("abc"), String::from("a*")), true);
        assert_eq!(Solution::is_match(String::from("abc"), String::from("a.")), false);
        assert_eq!(Solution::is_match(String::from("abc"), String::from("abc")), true);
        assert_eq!(Solution::is_match(String::from("abc"), String::from("a.*")), true);
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a*")), true);
        assert_eq!(Solution::is_match(String::from("ab"), String::from(".*")), true);
        // assert_eq!(Solution::is_match(String::from("aab"), String::from("c*a*b*")), true);
    }
}