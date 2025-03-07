pub fn dp_rec_mc(amount: u32) -> u32 {
    let coins = [1, 2, 5, 10, 20, 30, 50, 100];

    let mut dp = vec![u32::MAX; (amount + 1) as usize];

    dp[0] = 0;

    for i in 1..=amount as usize {
        for &coin in &coins {
            if i >= coin as usize && dp[i - coin as usize] != u32::MAX {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == u32::MAX {
        return 0;
    }

    dp[amount as usize]
}
