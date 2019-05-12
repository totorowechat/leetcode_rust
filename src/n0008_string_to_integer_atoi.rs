pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_atoi(input: String) -> i32 {
        let (i32_min, i32_max) = (-2_i64.pow(31), 2_i64.pow(31) - 1);
        let mut result: i64 = 0;
        let mut minus = false;
        let mut get_num_sign = false;
        let mut num_matched = false;

        for ch in input.chars().into_iter() {
            if !num_matched {
                match (ch) {
                    ' ' => {
                        if get_num_sign {
                            return 0;
                        }
                    }
                    '-' => {
                        if get_num_sign {
                            return 0;
                        }
                        minus = true;
                        get_num_sign = true;
                    }
                    '+' => {
                        if get_num_sign {
                            return 0;
                        }
                        get_num_sign = true;
                    }
                    '0'...'9' => {
                        num_matched = true;
                        result = result * 10 + ch.to_digit(10).unwrap() as i64;
                    }
                    _ => return 0,
                }
            } else {
                match (ch) {
                    '0'...'9' => {
                        result = result * 10 + ch.to_digit(10).unwrap() as i64;
                        if result > i32_max {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }

        result = if minus { -result } else { result };
        if result > i32_max {
            return i32_max as i32;
        }
        if result < i32_min {
            return i32_min as i32;
        }
        return result as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
        assert_eq!(Solution::my_atoi("+-123".to_string()), 0);
        assert_eq!(Solution::my_atoi("+ 123".to_string()), 0);
    }
}
