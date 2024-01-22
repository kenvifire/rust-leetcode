use std::collections::HashMap;
use std::fs::read;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for num in  nums {
            match map.get(num) {
                None => map.insert(num, 1);
                Some(_) => return true
            }

        }
        return false

    }
}