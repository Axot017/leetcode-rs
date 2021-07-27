pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut start = 0;
        let mut end = height.len() - 1;

        while start < end {
            let s = height[start];
            let e = height[end];
            let height = std::cmp::min(s, e);
            let width = (end - start) as i32;

            let current = width * height;
            if current > max {
                max = current;
            }

            if s < e {
                start += 1;
            } else {
                end -= 1;
            }
        }

        return max;
    }
}
