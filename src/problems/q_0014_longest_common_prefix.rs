// 14. Longest Common Prefix - Easy

/**
 * 假设第一个字符串是最长公共前缀，然后依次与后面的字符串对比，每次比较后更新公共前缀
 */

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix = strs[0].clone();
        for s in &strs[1..] {
            let mut i = 0;
            let min_len = prefix.len().min(s.len());
            while i < min_len && prefix.as_bytes()[i] == s.as_bytes()[i] {
                i += 1;
            }
            prefix.truncate(i);

            if prefix.is_empty() {
                return String::new();
            }
        }
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            "fl",
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
    }
}
