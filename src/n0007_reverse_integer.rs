// pub fn reverse(x: i32) -> i32 {
//     let mut x = x;
//     let mut rev: i32 = 0;
//     // if (rev > INT_MAX/10 || (rev == INT_MAX / 10 && pop > 7)) return 0;
//     // if (rev < INT_MIN/10 || (rev == INT_MIN / 10 && pop < -8)) return 0;

//     while x != 0 {
//         let pop = x % 10;
//         x /= 10;

//         if rev > i32::max_value() || (rev == i32::max_value() / 10 && pop > 7) {
//             return 0;
//         }
//         if rev < i32::min_value() || (rev == i32::min_value() / 10 && pop < -8) {
//             return 0;
//         }
//         rev = rev * 10 + pop;
//     }
//     rev
// }
pub fn reverse(x: i32) -> i32 {
    let mut input: i64 = x as i64;
    let mut result: i64 = 0;
    let mut digit: i64 = 0;
    let base: i64 = 2;
    let upper_bound: i64 = base.pow(31) - 1;
    let lower_bound: i64 = -base.pow(31);
    while input != 0 {
        digit = input % 10;
        result = result * 10 + digit;
        input = input / 10;
    }
    if result > upper_bound || result < lower_bound {
        return 0;
    }
    result as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(reverse((-base.pow(31)) as i32), 0);
    }
}
