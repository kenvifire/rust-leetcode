struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        result.push(1);
        if row_index  == 0 {
            return result;
        }

        for i in 1..row_index + 1  {
            let mut prev= result[0];
            for j in 1..i   {
                let temp = result[j as usize];
                result[j as usize] = prev + result[j as usize];
                prev = temp;
            }
            result.push(1);
        }
        result

    }
}

#[test]
pub fn basic_test() {
    println!("---------------------");
    println!("{:#?}", Solution::get_row(1));
    println!("---------------------");
    println!("{:#?}", Solution::get_row(2));
    println!("---------------------");
    println!("{:#?}", Solution::get_row(3));
    println!("---------------------");
    println!("{:#?}", Solution::get_row(4));
    println!("---------------------");
    println!("{:#?}", Solution::get_row(5));
}