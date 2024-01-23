use std::cmp::Ordering;

struct Solution{}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }

        let mut res = Vec::new();
        for i in  0..nums.len(){
            res.push((nums[i], i));
        }
        res.sort_by(|a,b | {
            if a.0.cmp(&b.0) == Ordering::Equal {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        for i in 1..res.len() {
            if res[i-1].0 == res[i].0 && (res[i-1].1 as i32 -res[i].1 as i32).abs() <=k {
                return true
            }
        }
        false


    }
}
#[test]
pub fn test() {
    Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2);
}