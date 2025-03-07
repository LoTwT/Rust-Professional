pub fn goldbach_conjecture() -> String {
    let limit = 10_000;
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(limit as f64).sqrt() as usize {
        if is_prime[i] {
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let primes: Vec<usize> = (2..limit).filter(|&p| is_prime[p]).collect();

    let is_prime_fn = |n: usize| -> bool {
        if n < limit {
            return is_prime[n];
        }

        if n % 2 == 0 {
            return n == 2;
        }

        let sqrt_n = (n as f64).sqrt() as usize + 1;
        for &p in &primes {
            if p > sqrt_n {
                break;
            }
            if n % p == 0 {
                return false;
            }
        }
        true
    };

    let satisfies_conjecture = |n: usize| -> bool {
        let max_square = ((n / 2) as f64).sqrt() as usize + 1;

        for i in 0..=max_square {
            let square_part = 2 * i * i;
            if square_part >= n {
                break;
            }

            let remainder = n - square_part;
            if is_prime_fn(remainder) {
                return true;
            }
        }
        false
    };

    let mut results = Vec::new();
    let mut n = 9;

    while results.len() < 2 {
        if n % 2 == 1 && !is_prime_fn(n) {
            if !satisfies_conjecture(n) {
                results.push(n);
            }
        }
        n += 2;
    }

    format!("{},{}", results[0], results[1])
}
