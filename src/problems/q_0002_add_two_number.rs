// 2. Add Two Numbers - Medium

/**
 * 初始化头节点 dummy_head，以及指针节点 current 指向 dummy_head、定义变量 carry 表示进位
 * 循环遍历 l1 和 l2 并取出当前位的值（若为空则置为 0 ）
 * 计算当前位数之和 sum，并记录进位 carry，之后取余并创建新节点，继续移动链表指针
 */

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p = l1;
        let mut q = l2;

        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut carry = 0;

        while p.is_some() || q.is_some() || carry != 0 {
            let x = p.as_ref().map_or(0, |node| node.val);
            let y = q.as_ref().map_or(0, |node| node.val);

            let sum = x + y + carry;
            carry = sum / 10;

            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            if let Some(node) = p {
                p = node.next;
            }
            if let Some(node) = q {
                q = node.next;
            }
        }
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            to_linked_list(vec![7, 0, 8]),
            Solution::add_two_numbers(to_linked_list(vec![2, 4, 3]), to_linked_list(vec![5, 6, 4]))
        );
        assert_eq!(
            to_linked_list(vec![0]),
            Solution::add_two_numbers(to_linked_list(vec![0]), to_linked_list(vec![0]))
        );
        assert_eq!(
            to_linked_list(vec![8, 9, 9, 9, 0, 0, 0, 1]),
            Solution::add_two_numbers(
                to_linked_list(vec![9, 9, 9, 9, 9, 9, 9]),
                to_linked_list(vec![9, 9, 9, 9])
            )
        );
    }
}

pub fn to_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = current;
        current = Some(node);
    }
    current
}
