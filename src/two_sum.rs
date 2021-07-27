use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (index, value) in nums.iter().enumerate() {
        let needed = target - value;
        let result = map.get(&needed);
        if let Some(&i) = result {
            return vec![i, index as i32];
        } else {
            map.insert(*value, index as i32);
        }
    }
    return vec![];
}
