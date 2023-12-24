use std::collections::HashMap;
use std::ffi::c_char;

struct Solution();

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let map = HashMap::from([( '}', '{'), (')','('), (']','[')]);
        let bytes = s.as_bytes();

        for ch in bytes {
            let curr = *ch as char;
            match curr {
                '{' | '(' | '[' => stack.push(curr),
                '}' | ')' | ']' => {
                    if stack.is_empty() {
                        return false;
                    }

                    if stack.get(stack.len() - 1).unwrap()  == map.get(&curr).expect("not found") {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => (),
            }
        }

        return stack.is_empty();

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("[}")), false);
        assert_eq!(Solution::is_valid(String::from("(()")), false);
        assert_eq!(Solution::is_valid(String::from("]")), false);
    }
}