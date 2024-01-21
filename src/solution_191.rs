struct Solution {}

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut num = n;

        let mut n = 0;
        for i in 0..32 {
            n += (num & 0x01) as i32;
            num >>= 1;
        }

        n as i32

    }
}