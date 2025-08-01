// 8. String to Integer (atoi) - Medium

/**
 * 将字符串转为字节数组，先跳过所有前导空格
 * 然后判断是否有 '+' 或 '-' 符号记录正负号
 * 之后遍历接下来的数字字符依次转换为整数
 * 在转换的过程进行溢出检查防止超出 i32 范围
 */

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut i = 0;
        let len = bytes.len();

        while i < len && bytes[i] == b' ' {
            i += 1;
        }

        let mut sign = 1;
        if i < len {
            if bytes[i] == b'-' {
                sign = -1;
                i += 1;
            } else if bytes[i] == b'+' {
                i += 1;
            }
        }

        let mut result = 0;
        while i < len && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i32;

            if result > (i32::MAX - digit) / 10 {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }

            result = result * 10 + digit;
            i += 1;
        }
        result * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(42, Solution::my_atoi("42".to_string()));
        assert_eq!(-42, Solution::my_atoi(" -042".to_string()));
        assert_eq!(1337, Solution::my_atoi("1337c0d3".to_string()));
        assert_eq!(0, Solution::my_atoi("0-1".to_string()));
        assert_eq!(0, Solution::my_atoi("words and 987".to_string()));
    }
}
