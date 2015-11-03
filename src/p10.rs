pub fn solve() -> usize {
    let mut sum = 0;
    for p in primes_less_than(2_000_000) {
        sum += p;
    }
    return sum;
}

pub fn primes_less_than(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    for i in 2..n {
        if is_prime(i, &primes) {
            primes.push(i);
        }
    }
    return primes;
}

pub fn is_prime(n: usize, primes: &[usize]) -> bool {
    for p in primes {
        if n % p == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 142913828922);
}

#[test]
fn test_primes_less_than() {
    assert_eq!(primes_less_than(10), vec![2, 3, 5, 7]);
    assert_eq!(primes_less_than(23), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    assert_eq!(primes_less_than(29), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
}
