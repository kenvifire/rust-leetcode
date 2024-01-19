struct Solution{}

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;
        let chars = column_title.chars();
        let mut base = 26;;

        for char in chars  {
            result *= base;
            result += ((char as u8) - ('A' as u8)) as i32 + 1;
        }
        result

    }
}