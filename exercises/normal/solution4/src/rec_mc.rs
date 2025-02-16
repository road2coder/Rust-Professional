pub fn dp_rec_mc(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }

    let coins = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;
    for i in 1..=amount {
        // 检查是否直接匹配硬币面值
        if coins.binary_search(&i).is_ok() {
            dp[i as usize] = 1;
            continue;
        }
        let min_coins = coins
            .iter()
            .take_while(|&&c| c < i)
            .map(|&c| dp[(i - c) as usize])
            .min()
            .map(|i| i + 1)
            .unwrap();
        dp[i as usize] = min_coins;
    }

    dp[amount as usize]
}
