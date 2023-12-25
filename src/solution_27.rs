struct Solution();
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut dup = 0;
        for i in 0..nums.len() {
            if nums[i] == val {
                dup += 1;
            }else if i > 0 {
                nums[i - dup] = nums[i];
            }
        }

        (nums.len() - dup) as i32

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        let mut vec = Vec::from([0,1,2,2,3,0,4,2]);
        assert_eq!(Solution::remove_element(&mut vec, 2), 5);
        println!("{:?}", vec);

    }
}