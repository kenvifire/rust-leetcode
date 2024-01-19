struct Solution{}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut x = (0, 0);

        for num in nums  {
            if num == x.0 {
                x.1 += 1;
            } else {
                if x.1 > 0 {
                    x.1 -= 1;
                } else {
                    x.0 = num;
                    x.1 = 1;
                }
            }

        }
        x.0
    }
}