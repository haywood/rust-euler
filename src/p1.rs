pub fn solve() -> i64 {
    return compute_sum(1000);
}

fn compute_sum(limit: i64) -> i64 {
    let mut sum = 0;
    for n in 0..limit {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    return sum;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 233168);
}

#[test]
fn test_compute_sum() {
    assert_eq!(compute_sum(10), 23);
    assert_eq!(compute_sum(1000), 233168);
}
