// 4. Median of Two Sorted Arrays - Hard

/**
 * 在较短的数组上二分查找分界点 i, 另一个数组的分界点 (m + n + 1) / 2 - i
 * 1. 左半部分总长度为 (m + n + 1) / 2 (两数组长度之和为偶数则为一半, 为奇数则 +1)
 * 2. 分界点两侧的值需满足：两个数组的左半部分不大于右半部分 a_left <= b_right && a_right >= b_left
 *
 * 当找到满足条件的分界点 i 和 j 后：
 * 若总长度为奇数，中位数则是左半部分的最大值；若总长度为偶数，中位数则是左最大值和右最小值的平均值
 */

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() > nums2.len() {
            (&nums2, &nums1)
        } else {
            (&nums1, &nums2)
        };

        let (m, n) = (a.len(), b.len());
        let (mut left, mut right) = (0, m);

        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n + 1) / 2 - i;

            let a_left = if i == 0 { i32::MIN } else { a[i - 1] };
            let b_left = if j == 0 { i32::MIN } else { b[j - 1] };
            let left_max = a_left.max(b_left) as f64;

            let a_right = if i == m { i32::MAX } else { a[i] };
            let b_right = if j == n { i32::MAX } else { b[j] };
            let right_min = a_right.min(b_right) as f64;

            if a_left <= b_right && a_right >= b_left {
                return if (m + n) % 2 == 0 {
                    (left_max + right_min) / 2.0
                } else {
                    left_max
                };
            } else if a_left > b_right {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
    }
}
