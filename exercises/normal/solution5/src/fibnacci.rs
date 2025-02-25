pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold == 0 {
        return 0;
    }
    let (mut first, mut second) = (0, 1);
    let mut sum = first + second;
    loop {
        let next = first + second;
        if next > threshold {
            break;
        };
        if next & 1 == 1 {
            sum += next;
        };
        first = second;
        second = next;
    }
    sum
}
