// 11. Container With Most Water - Medium

/**
 * 使用两个指针 left 和 right 分别从数组的两端开始扫描
 * 每次扫描计算当前容器的容量 max_are = h * w = min(height[left], height[right]) * (right - left) 并记录最大面积
 * 记录当前最大面积后移动高度较低一方的指针直至双指针相遇
 */

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let h = height[left].min(height[right]);
            let w = (right - left) as i32;
            max_area = max_area.max(h * w);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(49, Solution::max_area([1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec()));
        assert_eq!(1, Solution::max_area([1, 1].to_vec()));
    }
}
