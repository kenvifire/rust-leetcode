use std::collections::HashMap;

struct Solution {

}
impl Solution {
   pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut result: Vec<i32> = Vec::new();
       let len = nums.len();

       for i in 0..len - 1 {
           for j in i+1 .. len {
               if nums[i] + nums[j] == target {
                   result.push(i as i32);
                   result.push(j as i32);
                   return  result;
               }
           }
       }
       result
   }

    pub fn tow_sum_optimized(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res : Vec<i32> = vec![];
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (idx, e)  in nums.iter().enumerate() {
            map.insert(*e, idx);
        }

        for (idx, num) in nums.iter().enumerate() {
            let value = target - *num;
            if map.contains_key(&value) && *map.get(&value).unwrap() != idx {
                res.push(idx as i32);
                res.push(*map.get(&value).unwrap() as i32);

                return res;
            }
        }
        res
    }
}


#[cfg(test)]
mod test {
    use std::mem::size_of;
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::two_sum(vec![1,2,3], 3);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
    }

    #[test]
    fn test_optimized() {
        let result = Solution::tow_sum_optimized(vec![1,2,3], 3);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
    }
}