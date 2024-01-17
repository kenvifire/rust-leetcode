struct Solution{}

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {

        let mut num = column_number;
        let mut result = String::new();

        while num > 0 {
            num -= 1;
            result.push(((num % 26) as u8 + 'A' as u8) as char);
            num /= 26;
        }
        result.chars().rev().collect::<String>()

    }
}