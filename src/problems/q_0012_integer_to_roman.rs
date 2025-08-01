// 12. Integer to Roman - Medium

/**
 * 定义两个数组将罗马数字 symbols 和阿拉伯数字 values 按从大到小顺序存储起来
 * 将传入的数字 num 与数组 values 中的值从大到小进行比较
 * 如果 num 大于等于当前值则减去该值，并把罗马字符拼接到结果字符串中直至 num == 0
 */
pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut result = String::new();
        let mut num = num;

        for (i, &val) in values.iter().enumerate() {
            while num >= val {
                num -= val;
                result.push_str(symbols[i]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!("MMMDCCXLIX", Solution::int_to_roman(3749));
        assert_eq!("LVIII", Solution::int_to_roman(58));
        assert_eq!("MCMXCIV", Solution::int_to_roman(1994));
    }
}
