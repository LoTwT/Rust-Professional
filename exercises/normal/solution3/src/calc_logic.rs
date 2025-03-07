pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0;
    }

    let mut no_match_prob = 1.0;
    for i in 0..n {
        no_match_prob *= (365.0 - i as f64) / 365.0;
    }

    let probability = 1.0 - no_match_prob;

    (probability * 10000.0).round() / 10000.0
}
