pub fn solve() -> Option<i64> {
    return largest_prime_factor(600851475143);
}

pub fn largest_prime_factor(n: i64) -> Option<i64> {
    if n == 1 {
        return Some(1);
    } if n == 0 {
        return None;
    } else if n < 0 {
        return largest_prime_factor(-n);
    }

    let mut primes = Vec::new();
    let mut p = n;
    let mut i = 2;
    while i < p {
        if is_prime(i, &primes) {
            primes.push(i);
            while p != i && p % i == 0 {
                p /= i;
            }
        }
        i += 1;
    }
    return Some(p);
}

pub fn is_prime(n: i64, primes_less_than_n: &[i64]) -> bool {
    for p in primes_less_than_n {
        if n % p == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), Some(6857));
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(largest_prime_factor(0), None);
    assert_eq!(largest_prime_factor(1), Some(1));
    assert_eq!(largest_prime_factor(2), Some(2));
    assert_eq!(largest_prime_factor(3), Some(3));
    assert_eq!(largest_prime_factor(4), Some(2));
    assert_eq!(largest_prime_factor(5), Some(5));
    assert_eq!(largest_prime_factor(6), Some(3));
    assert_eq!(largest_prime_factor(7), Some(7));
    assert_eq!(largest_prime_factor(8), Some(2));
    assert_eq!(largest_prime_factor(9), Some(3));
    assert_eq!(largest_prime_factor(10), Some(5));
    assert_eq!(largest_prime_factor(11), Some(11));
    assert_eq!(largest_prime_factor(13195), Some(29));
    assert_eq!(largest_prime_factor(-13195), Some(29));
}
