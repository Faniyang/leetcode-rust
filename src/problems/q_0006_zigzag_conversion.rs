// 6. Zigzag Conversion - Medium

/**
 * 本题需要定义一个 Vec<String> 类型的动态字符数组
 * 根据给定的行数，将字符按 Zigzag 规则逐行添加到对应的行中
 * 当遍历到第一行或最后一行需要转换方向
 * 最后将所得的动态字符数组拼接起来并返回
 */

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }

        let mut rows = vec![String::new(); num_rows as usize];
        let mut curr_row = 0;
        let mut going_down = false;

        for ch in s.chars() {
            rows[curr_row].push(ch);
            if curr_row == 0 || curr_row == (num_rows - 1) as usize {
                going_down = !going_down;
            }
            curr_row = if going_down {
                curr_row + 1
            } else {
                curr_row - 1
            };
        }
        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            "PAHNAPLSIIGYIR",
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );

        assert_eq!(
            "PINALSIGYAHRPI",
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );

        assert_eq!("A", Solution::convert("A".to_string(), 1));
    }
}
