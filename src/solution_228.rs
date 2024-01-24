use std::fmt::format;

struct Solution{}
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {

        let mut res = Vec::<String>::new();
        if nums.len() < 1 {
            return res
        }

        let mut a = nums[0];
        let mut b = nums[0];

        for i in 1..nums.len() {
            if nums[i]  == b + 1 {
               b = b+1;
            } else {
                if a == b {
                    res.push(format!("{}", a));
                } else {
                    res.push(format!("{}->{}", a, b));
                }
                a = nums[i];
                b = nums[i];
            }
        }

        if a == b {
            res.push(format!("{}", a));
        } else {
            res.push(format!("{}->{}", a, b));
        }

        res

    }
}