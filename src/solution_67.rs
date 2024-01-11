struct Solution{}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();

        let mut i = 0;
        let mut carry = 0;
        let mut byte_a = a.as_bytes();
        let mut byte_b = b.as_bytes();
        let len_a = byte_a.len() - 1;
        let len_b = byte_b.len() - 1;

        while i < a.len() && i < b.len() {
            let num_a = Solution::byte_to_num(byte_a[len_a - i]);
            let num_b = Solution::byte_to_num(byte_b[len_b - i]);

            if num_a + num_b + carry > 1 {
                result.push(Solution::num_to_char(num_a + num_b + carry - 2));
                carry = 1;
            } else {
                result.push(Solution::num_to_char(num_a + num_b + carry));
                carry = 0;
            }

            i+=1;
        }

        let mut c = &byte_a;
        if b.len() > a.len() {
            c = &byte_b;
        }

        while i < c.len() {
            let num_a = Solution::byte_to_num(c[c.len() -1 - i]);
            if num_a + carry > 1 {
                result.push(Solution::num_to_char(num_a + carry - 2));
                carry = 1;
            } else {
                result.push(Solution::num_to_char(num_a + carry));
                carry = 0;
            }
            i+=1;
        }

        if carry > 0 {
            result.push('1');
        }

        result.chars().rev().collect::<String>()

    }

    fn byte_to_num(b : u8) -> u32 {
        match b as char {
            '1' => 1,
            _ => 0
        }
    }

    fn num_to_char(n : u32) -> char {
        match n {
            1 => '1',
            _ => '0'
        }
    }
}

mod test {
    use super::*;
    #[test]
    pub fn test_basic() {
        assert_eq!(Solution::add_binary("11".to_string(), "1".to_string()), "100");

    }

}