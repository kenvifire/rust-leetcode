struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min: Vec<i32> = Vec::new();
        let mut curr_min = prices[0];
        let mut max = 0;

        for i in 0..prices.len() {
            if prices[i]  < curr_min {
                curr_min = prices[i];
            }
            min.push(curr_min);
        }

        for i in  0.. prices.len() {
            if prices[i] - min[i] > max {
                max = prices[i] - min[i];
            }
        }
        max



    }
}