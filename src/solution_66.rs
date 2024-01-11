struct Solution{}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        let mut carry = 1;
        let len = digits.len();

        for i  in  0..digits.len() {
            let curr = digits[len - i -1]  + carry;

            if curr >= 10 {
               result.push(curr - 10);
               carry = 1;
            } else {
                result.push(curr);
                carry = 0;
            }
        }
        
        if carry > 0 {
            result.push(carry);
        }


        result.reverse();
        result
    }
}

mod test{
    use super::*;
    #[test]
    pub fn test_basic() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
    }
}