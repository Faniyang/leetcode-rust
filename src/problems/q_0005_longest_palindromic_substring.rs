/**
 * 5. Longest Palindromic Substring - Medium
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 * Example:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 */

/**
 * 回文的特性是从中心到两边对称
 * 因此可以从字符串的每一个位置出发，然后向左右两边进行扩展对比，直到两边字符不相等为止
 * 我们需要分别考虑奇数长度的扩展和偶数长度的扩展，并在遍历的过程中记录下最长的回文子串的起始位置
 */

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s_bytes = s.as_bytes();
        let (mut start, mut end) = (0, 0);

        for i in 0..s.len() {
            let (l1, r1) = Self::expand_from_center(&s_bytes, i as isize, i as isize);
            let (l2, r2) = Self::expand_from_center(&s_bytes, i as isize, i as isize + 1);

            if r1 - l1 > end - start {
                start = l1;
                end = r1;
            }
            if r2 - l2 > end - start {
                start = l2;
                end = r2;
            }
        }

        s[start..end].to_string()
    }

    fn expand_from_center(s: &[u8], mut left: isize, mut right: isize) -> (usize, usize) {
        let len = s.len() as isize;
        while left >= 0 && right < len && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }

        ((left + 1) as usize, right as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
        assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
    }
}
