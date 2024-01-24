use std::fs::read;

struct Solution{}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut num = n;
        if n <= 0  {
            return false;
        }
        let mut cnt = 0;

        while num > 0 {
            cnt += (num & 0x01);
            if cnt > 1 {
                return false;
            }
            num >>= 1;
        }

        cnt == 1


    }
}