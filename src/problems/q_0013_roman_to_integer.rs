// 13. Roman to Integer - Easy

/**
 * 定义一个罗马数字与阿拉伯数字一一对应的哈希表，然后遍历传入的罗马字符串
 * 将当前字符 curr 的数值小于下一个字符 next 的数值进行比较
 * 如果是减法罗马字符组合(eg. IV)，从 total 减去当前数值 curr 否则加上
 */

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;

        for i in 0..chars.len() {
            let curr = map[&chars[i]];
            let next = if i + 1 < chars.len() {
                map[&chars[i + 1]]
            } else {
                0
            };
            if curr < next {
                total -= curr;
            } else {
                total += curr;
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
