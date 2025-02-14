pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut nums = vec![1, 1];
    let (mut first, mut second) = (1, 1);
    loop {
        let sum_prev_two = first + second;
        if sum_prev_two > threshold {
            break;
        } else if sum_prev_two & 1 == 1 {
            nums.push(sum_prev_two);
        }
        first = second;
        second = sum_prev_two;
    }
    nums.iter().sum()
}
