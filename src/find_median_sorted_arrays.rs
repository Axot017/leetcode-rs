pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.is_empty() || nums2.is_empty() {
            let not_empty = if nums1.is_empty() { &nums2 } else { &nums1 };
            let half = not_empty.len() / 2;

            if not_empty.len() % 2 == 1 {
                return not_empty[half] as f64;
            } else {
                return (not_empty[half - 1] + not_empty[half]) as f64 / 2.0;
            }
        }

        let (nums1, nums2) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let len1 = nums1.len() as i32;
        let len2 = nums2.len() as i32;
        let total_len = len1 + len2;
        let half_len = total_len / 2;

        let mut left = 0;
        let mut right = len1;

        loop {
            let l1 = (left + right) / 2;
            let l2 = half_len - l1;

            let left1 = if l1 - 1 >= 0 {
                nums1[(l1 - 1) as usize]
            } else {
                i32::MIN
            };
            let right1 = if l1 < len1 {
                nums1[l1 as usize]
            } else {
                i32::MAX
            };
            let left2 = if l2 - 1 >= 0 {
                nums2[(l2 - 1) as usize]
            } else {
                i32::MIN
            };
            let right2 = if l2 < len2 {
                nums2[l2 as usize]
            } else {
                i32::MAX
            };

            if left1 <= right2 && left2 <= right1 {
                if total_len % 2 == 0 {
                    return (std::cmp::max(left1, left2) as f64
                        + std::cmp::min(right1, right2) as f64) as f64
                        / 2.0;
                } else {
                    return std::cmp::min(right1, right2) as f64;
                }
            }
            if left1 > right2 {
                right = l1 - 1;
            } else {
                left = l1 + 1;
            }
        }
    }
}
