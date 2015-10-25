pub fn solve() -> usize {
    return nth_prime(10_001);
}

pub fn nth_prime(n: usize) -> usize {
    if n == 0 { panic!("can't get the 0th prime"); }
    return primes(n).pop().unwrap();
}

pub fn primes(n: usize) -> Vec<usize> {
    let mut ps = Vec::with_capacity(n);
    let mut x = 2;
    while ps.len() < n {
        if is_prime(x, &ps) {
            ps.push(x.clone());
        }
        x += 1;
    }
    return ps;
}

pub fn is_prime(x: usize, primes: &[usize]) -> bool {
    for p in primes {
        if x % p == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 104_743);
}

#[test]
fn test_nth_prime() {
    assert_eq!(nth_prime(6), 13);
    assert_eq!(nth_prime(7), 17);
    assert_eq!(nth_prime(8), 19);
    assert_eq!(nth_prime(9), 23);
}

#[test]
fn test_primes() {
    assert_eq!(primes(6), vec![2, 3, 5, 7, 11, 13]);
    assert_eq!(primes(7), vec![2, 3, 5, 7, 11, 13, 17]);
    assert_eq!(primes(8), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    assert_eq!(primes(9), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
}
