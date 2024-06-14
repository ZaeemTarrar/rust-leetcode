pub mod sol2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: Vec<(i32, i32)> = Vec::new();
        for (k, &v) in nums.iter().enumerate() {
            if let Some(i) = map.iter().position(|&(k1, _)| k1 == v) {
                return vec![i as i32, k as i32];
            }
            map.push((target - &v, k as i32));
        }
        return vec![];
    }
}
