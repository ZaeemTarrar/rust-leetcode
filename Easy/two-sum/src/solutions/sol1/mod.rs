pub mod sol1 {
    use std::collections::HashMap;
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (k, &v) in nums.iter().enumerate() {
            if let Some(&i) = map.get(&v) {
                return vec![i as i32, k as i32];
            }
            map.insert(target - &v, k as i32);
        }
        vec![]
    }
}
