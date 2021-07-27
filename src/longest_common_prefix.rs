pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = "".to_string();
        let mut i = 0;
        loop {
            let chars: Vec<Option<char>> = strs.iter().map(|s| s.chars().nth(i)).collect();
            let mut iter = chars.iter();
            let first = iter.next().unwrap_or(&None);
            let same = iter.all(|c| c == first);
            if same && first.is_some() {
                i += 1;
                result.push(first.unwrap())
            } else {
                return result;
            }
        }
    }
}
