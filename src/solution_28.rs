struct Solution();


impl Solution {
    pub fn str_str(haystack: String, needle: &str) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        let mut idx = -1;
        let hay_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        for i in 0..(hay_bytes.len() - needle_bytes.len() + 1) {
            let mut flag = true;
            for j in 0..needle_bytes.len() {
                if needle_bytes[j] != hay_bytes[i + j] {
                    flag = false;
                    break;
                }
            }
            idx += 1;
            if flag {
                return idx;
            }

        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(Solution::str_str(String::from("aaa"),"aa"), 0);
        assert_eq!(Solution::str_str(String::from("abaa"),"aa"), 2);
        assert_eq!(Solution::str_str(String::from("abaa"),"bbaa"), -1);
    }
}