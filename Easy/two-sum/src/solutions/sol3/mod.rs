pub mod sol3 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<(usize, i32)> = nums
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        nums.sort_unstable_by_key(|&(_, n)| n);
        for (k, (i, ni)) in nums.iter().enumerate() {
            match nums[k + 1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj) {
                Ok(jj) => {
                    return vec![*i as i32, nums[jj + k + 1].0 as i32];
                }
                Err(_) => {}
            }
            println!();
        }
        unreachable!("Error: this place should be unreachable");
        return vec![0, 0];
    }
}
