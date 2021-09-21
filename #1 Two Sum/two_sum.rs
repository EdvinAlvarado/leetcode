use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32,usize> = HashMap::new();
        for (i,n) in nums.into_iter().enumerate() {
            match map.get(&(target-n)) {
                None => {map.insert(n, i);},
                Some(&j) => return vec![j as i32, i as i32],
            }
        }
        vec![]
    }
}
