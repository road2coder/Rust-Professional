pub fn new_birthday_probability(n: u32) -> f64 {
    // 没人重复的概率为P(n) = C(365, n) = 365! / (365 - n) * 365 ^ n
    // 使用dp解决高阶幂的问题: P(n + 1) / P(n) = (365 - n) / 365
    let mut probability = 1.0;
    for i in 0..n {
        probability *= (365.0 - i as f64) / 365.0;
    }
    1.0 - probability
}
