use std::any::type_name;
use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();

        for ch in s.chars() {
            match s_map.get(&ch) {
                None => {
                    s_map.insert(ch, 1);
                },
                Some(cnt) => {
                    s_map.insert(ch, cnt + 1);
                },
            }
        }

        for ch in t.chars() {
            match t_map.get(&ch) {
                None => {
                    t_map.insert(ch, 1);
                },
                Some(cnt) => {
                    t_map.insert(ch, cnt+1);
                },
            }
        }

        if s_map.len() != t_map.len() {
            return false;
        }

        for (k, v) in &t_map {
            if !s_map.contains_key(k) || !s_map.get(k).eq(&t_map.get(k)) {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_basic() {
    Solution::is_anagram("aa".to_string(), "bb".to_string());
    Solution::is_anagram("ab".to_string(),"ba".to_string());
}