// 3. Longest Substring Without Repeating Characters - Medium

/**
 * 遍历字符串 s, 使用滑动窗口 [start..end]
 * 用 HashMap 记录字符上次出现的位置，遇到重复字符时更新 start
 * 每次迭代计算并更新 max_len
 */

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut map, mut start, mut max_len) = (HashMap::new(), 0, 0);

        for (end, ch) in s.chars().enumerate() {
            if let Some(&i) = map.get(&ch) {
                start = start.max(i + 1);
            }
            map.insert(ch, end);
            max_len = max_len.max(end - start + 1);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_string())
        );
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
    }
}
