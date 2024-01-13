struct Solution{}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        if n == 1 {
            return 1;
        }

        let mut result = vec![];
        for i in 0..(n + 1) {
            result.push(0);
        }
        result[0] = 1;
        result[1] = 1;

        for i in 2..(n+1) as usize {
            result[i] = result[i-1] + result[i-2];

        }
        result[n as usize]

    }
}

#[test]
pub fn test_basic() {
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}