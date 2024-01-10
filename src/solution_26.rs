struct Solution{}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut dup = 0;
        let mut curr_val = nums[0];
        for i in 1..nums.len() {
            if nums[i] == curr_val {
                dup += 1;
            } else {
                curr_val = nums[i];
            }

            if i > 0 {
                nums[i - dup] = nums[i];
            }
        }

        (nums.len() - dup) as i32
    }
}