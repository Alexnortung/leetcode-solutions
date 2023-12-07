use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // number and index
    let mut map: HashMap<i32, u16> = HashMap::with_capacity(nums.len());

    for (i, num) in nums.iter().enumerate() {
        let search = target - num;
        if let Some(&other_index) = map.get(&search) {
            return vec![i as i32, other_index as i32];
        }
        map.insert(num.clone(), i as u16);
    }

    vec![]
}
