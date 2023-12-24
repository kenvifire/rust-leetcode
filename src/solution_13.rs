struct Solution();

struct ByteWrapper<'a> {
    curr_pos: usize,
    bytes: &'a [u8],
}

impl<'a> ByteWrapper<'a> {
    fn next(&mut self) -> Option<char> {
        if self.has_next() {
            self.curr_pos += 1;
            Some(self.bytes[self.curr_pos - 1] as char)
        } else {
            None
        }
    }

    fn has_next(&self) -> bool {
        self.curr_pos < self.bytes.len()
    }

    fn peek(&self) -> Option<char> {
        if self.has_next() {
            Some(self.bytes[self.curr_pos] as char)
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.curr_pos += 1;
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut bytes = ByteWrapper {
            bytes: s.as_bytes(),
            curr_pos: 0,
        };

        let mut num = 0;

        while bytes.has_next() {
            match bytes.next() {
                Some('I') => {
                    match bytes.peek() {
                        Some('V') => {
                            num += 4;
                            bytes.advance()
                        }
                        Some('X') => {
                            num += 9;
                            bytes.advance();
                        }
                        Some(_) | None => num += 1,

                    }
                }
                Some('V') => num += 5,

                Some('X') => {
                    match bytes.peek() {
                        Some('L') => {
                            num += 40;
                            bytes.advance()
                        }
                        Some('C') => {
                            num += 90;
                            bytes.advance()
                        }
                        Some(_) | None  => num += 10,
                    }
                }
                Some('L') => num += 50,

                Some('C') => {
                    match bytes.peek() {
                        Some('D') => {
                            num += 400;
                            bytes.advance()
                        }
                        Some('M') => {
                            num += 900;
                            bytes.advance()
                        }
                        Some(_) | None => num += 100,
                    }
                }

                Some('D') => num += 500,

                Some('M') => num += 1000,

                _ => num += 0,
            }
        }
        num
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn basic_test() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        // assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        // assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        // assert_eq!(Solution::roman_to_int(String::from("LVIII")), 3);
        // assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}