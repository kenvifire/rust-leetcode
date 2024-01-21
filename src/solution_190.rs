struct Solution{}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut res = 0;
        let mut num = x;

        for i in 0..32 {
            res <<= 1;
            res += (num & 0x01);
            num >>= 1;
        }
        res

    }
}