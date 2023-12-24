struct Solution();

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }

        let mut res = String::new();

        let mut idx = 0;
        let mut flag = false;
        let mut curr_ch: Option<char> = None;
        loop {
            for str in strs.iter() {
                if idx >= str.len() {
                    flag = true;
                    break;
                }

                let ch = str.as_bytes()[idx] as char;

                match curr_ch {
                    Some(prev) => {
                       if ch != prev {
                           flag = true;
                           break;
                       }
                    }
                    None => {
                        curr_ch = Some(ch)
                    }
                }
            }
            if flag {
                break;
            }
            res.push(curr_ch.expect("value"));
            curr_ch = None;
            idx += 1;
        }
        res
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
        assert_eq!(Solution::longest_common_prefix(vec![String::from("abc")]), "abc");
        assert_eq!(Solution::longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]), "fl");
        assert_eq!(Solution::longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]), "");
        assert_eq!(Solution::longest_common_prefix(vec![String::from("ab"), String::from("abc"), String::from("abcd")]), "ab");
        assert_eq!(Solution::longest_common_prefix(vec![String::from("abcd"), String::from("abc"), String::from("ab")]), "ab");
    }
}