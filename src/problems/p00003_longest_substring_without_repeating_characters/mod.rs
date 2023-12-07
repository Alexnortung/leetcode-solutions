use super::Solution;
use std::{char, collections::HashMap};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut max = 0;
        let mut map: HashMap<char, i32> = HashMap::new();

        if s.len() == 0 {
            return 0;
        }

        for (i, c) in s.chars().enumerate() {
            let last_index_option = map.get(&c).cloned();
            map.insert(c, i as i32);
            if let Some(last_index) = last_index_option {
                if last_index >= start {
                    start = map
                        .values()
                        .filter(|x| x > &&last_index)
                        .min()
                        .unwrap_or(&start)
                        .clone();
                }
            }
            max = max.max(i as i32 - start);
        }
        max + 1
    }
}
