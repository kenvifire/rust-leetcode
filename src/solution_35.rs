struct Solution();

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {


        let mut start:i32 = 0;
        let mut end:i32 = (nums.len() - 1) as i32;

        let mut mid:i32 = 0;
        let mut ans = nums.len() as i32;
        while start <= end {
            mid = start + (end - start)/2;

            if target <= nums[mid as usize]  {
                ans = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],0), 0) ;
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],1), 0) ;
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],2), 1) ;
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],3), 1) ;
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],4), 2) ;
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],5), 2) ;
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],6), 3) ;
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6],7), 4) ;
        assert_eq!(Solution::search_insert(vec![1],1), 0) ;
    }
}