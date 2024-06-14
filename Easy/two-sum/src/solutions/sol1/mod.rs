pub mod sol1 {
    use std::collections::HashMap;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (k, v) in nums.iter().enumerate() {
            if map.contains_key(v) {
                return vec![k as i32, *map.get(v).unwrap() as i32];
            }
            map.insert(target - v, k);
        }
        return vec![];
    }
}
