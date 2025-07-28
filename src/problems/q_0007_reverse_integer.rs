/**
 * 7. Reverse Integer - Medium
 *
 * Given a signed 32-bit integer x, return x with its digits reversed.
 * If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
 *
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *
 * Example
 *
 * Input: x = 123
 * Output: 321
 *
 */

/**
 * 通过对传入的整数进行逐位提取，用 %10 获得当前位，用 /10 去掉当前位
 * 每次将结果累积到 rev = rev * 10 + pop，逐步构建反转后的数字
 *
 * 由于输入是 i32 类型，范围是[-2³¹, 2³¹ - 1]），所以反转过程中可能会发生溢出
 * 为了避免溢出，需要针对这种 case 添加边界条件
 * 在每次更新 rev 之前，需要提前判断反转时是否会向上或向下溢出
 * i32::MAX = 2147483647
 * i32::MIN = -2147483648
 */

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rev: i32 = 0;

        while x != 0 {
            let pop = x % 10;
            x /= 10;

            if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) {
                return 0;
            }
            if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) {
                return 0;
            }

            rev = rev * 10 + pop;
        }

        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(21, Solution::reverse(120));
    }
}
