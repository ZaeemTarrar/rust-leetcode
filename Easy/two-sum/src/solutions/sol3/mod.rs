pub mod sol3 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<(usize, i32)> = nums
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        nums.sort_unstable_by_key(|&(_, v)| v);
        for (i, (k, v)) in nums.iter().enumerate() {
            match nums[i + 1..].binary_search_by_key(&(target - *v), |&(_, b)| b) {
                Ok(j) => {
                    return vec![*k as i32, nums[j + i + 1].0 as i32];
                }
                Err(_) => {}
            }
        }
        unreachable!();
    }
}
