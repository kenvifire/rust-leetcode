struct Solution{}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut num = 0;
        for x in nums {
           num ^= x;

        }
        num

    }
}