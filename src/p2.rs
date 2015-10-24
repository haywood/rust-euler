pub fn solve() -> i64 {
    return sum_of_fibonacci_numbers_leq(4_000_000);
}

pub fn sum_of_fibonacci_numbers_leq(limit: i64) -> i64 {
    // TODO: suspect that something clever can be done here using the closed form
    let mut n = 2;
    let mut n1 = 1;
    let mut n2;
    let mut sum = 0;
    loop {
        if n > limit {
            return sum;
        } else if n % 2 == 0 {
            sum += n;
        }
        n2 = n1;
        n1 = n;
        n = n1 + n2;
    }
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 4613732);
}

#[test]
fn test_sum_of_fibonacci_numbers_leq() {
    assert_eq!(sum_of_fibonacci_numbers_leq(1), 0);
    assert_eq!(sum_of_fibonacci_numbers_leq(2), 2);
    assert_eq!(sum_of_fibonacci_numbers_leq(3), 2);
    assert_eq!(sum_of_fibonacci_numbers_leq(5), 2);
    assert_eq!(sum_of_fibonacci_numbers_leq(8), 10);
    assert_eq!(sum_of_fibonacci_numbers_leq(13), 10);
    assert_eq!(sum_of_fibonacci_numbers_leq(21), 10);
    assert_eq!(sum_of_fibonacci_numbers_leq(34), 44);
    assert_eq!(sum_of_fibonacci_numbers_leq(55), 44);
    assert_eq!(sum_of_fibonacci_numbers_leq(89), 44);
}
