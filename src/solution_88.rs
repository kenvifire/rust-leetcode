struct Solution{}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut result: Vec<i32> = Vec::new();

        let mut idx_m = 0;
        let mut idx_n = 0;
        while idx_m < m || idx_n < n {
            if idx_m < m && idx_n < n {
                if nums1[idx_m as usize] <= nums2[idx_n as usize] {
                    result.push(nums1[idx_m as usize]);
                    idx_m += 1;
                } else {
                    result.push(nums2[idx_n as usize]);
                    idx_n += 1;
                }


            } else if (idx_m < m) {
                result.push(nums1[idx_m as usize]);
                idx_m += 1;

            } else if (idx_n < n) {
                result.push(nums2[idx_n as usize]);
                idx_n += 1;
            }
        }

        for i in 0..(m+n) {
            nums1[i as usize] = result[i as usize];
        }
    }

    pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx = (m + n -1);
        let mut idx_m = (m - 1);
        let mut idx_n = (n - 1);

        while idx >= 0 {

            if idx_m >= 0  && idx_n >= 0 {
                if nums1[idx_m as usize] > nums2[idx_n as usize] {
                    nums1[idx as usize] = nums1[idx_m as usize];
                    idx_m -= 1 ;
                } else {
                    nums1[idx as usize] = nums2[idx_n as usize];
                    idx_n -= 1;
                }
            } else if idx_m >= 0 {
                nums1[idx as usize] = nums1[idx_m as usize];
                idx_m -= 1 ;
            } else if idx_n >= 0 {
                nums1[idx as usize] = nums2[idx_n as usize];
                idx_n -= 1;
            }
            idx -= 1;
        }
    }
}

#[test]
pub fn test_basic() {
    let mut nums1 = Vec::new();
    nums1.push(1);
    nums1.push(1);

    let mut num2 = Vec::new();
    num2.push(1);
    Solution::merge2(&mut nums1, 1, &mut num2, 1);
}