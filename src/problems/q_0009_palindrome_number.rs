// 9. Palindrome Number - Easy

/**
 * 直接反转整个数字需要考虑数字反转后是否会向上溢出
 * 可以只反转数字的一半，既可以避免溢出，也能准确判断数字是否为回文
 */

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut x = x;
        let mut rev = 0;

        while x > rev {
            rev = rev * 10 + x % 10;
            x /= 10;
        }

        rev == x || rev / 10 == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(false, Solution::is_palindrome(10));
        assert_eq!(true, Solution::is_palindrome(0));
    }
}
