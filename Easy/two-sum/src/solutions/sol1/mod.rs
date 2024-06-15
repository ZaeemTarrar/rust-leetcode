pub mod sol1 {
    use std::collections::HashMap;
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (k, &v) in nums.iter().enumerate() {
            match map.get(&v) {
                Some(&i) => {
                    return vec![i as i32, k as i32];
                }
                None => {
                    map.insert(target - &v, k as i32);
                }
            }
        }
        unreachable!()
    }
}
