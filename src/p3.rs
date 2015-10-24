pub fn solve() -> Option<i64> {
    return largest_prime_factor(600851475143);
}

pub fn largest_prime_factor(n: i64) -> Option<i64> {
    // TODO: performs pretty poorly timewise on the input from solve
    if n == 1 {
        return Some(1);
    } if n == 0 {
        return None;
    } else if n < 0 {
        return largest_prime_factor(-n);
    }

    let limit = 1 + (n as f64).sqrt().ceil() as i64;
    let mut primes = Vec::with_capacity(approx_primes_leq(limit) as usize);
    for i in 2..limit {
        if is_prime(i, &primes) {
            primes.push(i);
        }
    }
    for p in primes.iter().rev() {
        if n % p == 0 {
            return Some(p.clone());
        }
    }
    return None;
}

pub fn is_prime(n: i64, primes_less_than_n: &[i64]) -> bool {
    for p in primes_less_than_n {
        if n % p == 0 {
            return false;
        }
    }
    return true;
}

pub fn approx_primes_leq(n: i64) -> f64 {
    let x = n as f64;
    return 1.25506 * x / x.ln()
}

#[test]
fn test_solve() {
    assert_eq!(solve(), Some(6857));
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(13195), Some(29));
}
