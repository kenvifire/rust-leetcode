struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        result.push(vec![1]);
        for i in 1..num_rows  {
            let mut cur: Vec<i32> = Vec::new();
            cur.push(1);
            for j in 1..i  {
                cur.push(result[i as usize -1][j as usize-1] + result[i as usize -1][j as usize]) ;
            }
            cur.push(1);
            result.push(cur);
        }
        result

    }
}

#[test]
pub fn basic_test() {
    println!("---------------------");
    println!("{:#?}", Solution::generate(1));
    println!("---------------------");
    println!("{:#?}", Solution::generate(2));
    println!("---------------------");
    println!("{:#?}", Solution::generate(3));
    println!("---------------------");
    println!("{:#?}", Solution::generate(4));
    println!("---------------------");
    println!("{:#?}", Solution::generate(5));
}