pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut nums = vec![1, 1];
    loop {
        let sum_prev_two = nums[nums.len() - 2..].iter().sum();
        if sum_prev_two > threshold {
            break;
        }
        nums.push(sum_prev_two);
    }
    nums.iter().filter(|i| **i & 1 == 1).sum()
}
