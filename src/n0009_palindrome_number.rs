pub struct Solution {}

// submission codes start here

// TODO: not optimal, we only have to revert half of the string
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        };
        let (mut copy, mut rev, mut r, mut n) = (x, 0, 0, x);
        while n > 0 {
            r = n % 10;
            rev = rev * 10 + r;
            n /= 10;
        }
        if copy == rev {
            return true;
        };
        return false;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}
