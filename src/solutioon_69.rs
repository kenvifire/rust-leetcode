struct Solution{}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0
        }

        let x1 = x as i64;
        let mut root = 1;
        for i in 1.. (x/2 + 1) {
           if i as i64 * i as i64 <= x1 {
               root = i;
           } else {
               break;
           }
        }
        root

    }
}

#[test]
pub fn test_basic() {
    assert_eq!(Solution::my_sqrt(1), 1);
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
    assert_eq!(Solution::my_sqrt(9), 3);
    assert_eq!(Solution::my_sqrt(25), 5);
    assert_eq!(Solution::my_sqrt(26), 5);
    assert_eq!(Solution::my_sqrt(2147395600), 46340);
}
