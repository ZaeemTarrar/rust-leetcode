pub mod sol2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: Vec<(i32, i32)> = Vec::new();
        for (k, &v) in nums.iter().enumerate() {
            match map.iter().position(|&(k1, _)| k1 == v) {
                Some(i) => {
                    return vec![i as i32, k as i32];
                }
                None => {
                    map.push((target - &v, k as i32));
                }
            }
        }
        unreachable!()
    }
}
