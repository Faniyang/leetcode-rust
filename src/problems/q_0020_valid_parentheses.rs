// 20. Valid Parentheses - Easy

/**
 * 遍历字符串中的每个字符
 * 如果是括号的左半部分('(', '{', '[')就入栈
 * 如果是括号的右半部分(')', '}', ']')则检查栈顶
 *      1. 如果栈顶是对应的左括号则继续遍历
 *      2. 如果为空或者不是对应的左括号就返回 false
 * 遍历完后，如果栈为空，则说明都匹配成功返回 true，否则说明栈内还有未闭合的左括号返回 false
 */

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                ')' | '}' | ']' => {
                    if !matches!(
                        (ch, stack.pop()),
                        (')', Some('(')) | ('}', Some('{')) | (']', Some('['))
                    ) {
                        return false;
                    }
                }

                _ => {}
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
        assert_eq!(false, Solution::is_valid("(]".to_string()));
        assert_eq!(true, Solution::is_valid("([])".to_string()));
        assert_eq!(false, Solution::is_valid("([)]".to_string()));
    }
}
