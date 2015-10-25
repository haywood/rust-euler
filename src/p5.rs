pub fn solve() -> i64 {
    return least_common_multiple(20);
}

pub fn least_common_multiple(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut p = 1;
    for i in 2..n+1 {
        let gcd = greatest_common_divisor(i, p);
        p = i * p / gcd;
    }
    return p;
}

pub fn greatest_common_divisor(mut a: i64, mut b: i64) -> i64 {
    if a == 0 || b == 0 {
        panic!("cannot compute greatest common divisor with zero")
    }
    // modular arithemtic on negative numbers is weird
    if a < 0 {
        a = a.abs();
    }
    if b < 0 {
        b = b.abs();
    }
    if b < a {
        let tmp = b;
        b = a;
        a = tmp;
    }
    if a == 1 {
        return 1;
    }
    let mut r = b % a;
    while r != 0 {
        b = a;
        a = r;
        r = b % a;
    }
    return a;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 232_792_560);
}

#[test]
fn test_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(1, 2), 1);
    assert_eq!(greatest_common_divisor(3, 4), 1);
    assert_eq!(greatest_common_divisor(17, 21), 1);
    assert_eq!(greatest_common_divisor(14, 21), 7);
    assert_eq!(greatest_common_divisor(21, 14), 7);
    assert_eq!(greatest_common_divisor(27, 81), 27);
    assert_eq!(greatest_common_divisor(27, 72), 9);
    assert_eq!(greatest_common_divisor(-27, 72), 9);
    assert_eq!(greatest_common_divisor(72, -27), 9);
}

#[test]
#[should_panic(expected = "cannot compute greatest common divisor with zero")]
fn test_greatest_common_divisor_with_zero() {
    greatest_common_divisor(0, 12);
}

#[test]
fn test_least_common_multiple() {
    assert_eq!(least_common_multiple(0), 0);
    assert_eq!(least_common_multiple(1), 1);
    assert_eq!(least_common_multiple(2), 2);
    assert_eq!(least_common_multiple(3), 6);
    assert_eq!(least_common_multiple(4), 12);
    assert_eq!(least_common_multiple(5), 60);
    assert_eq!(least_common_multiple(6), 60);
    assert_eq!(least_common_multiple(7), 420);
    assert_eq!(least_common_multiple(8), 840);
    assert_eq!(least_common_multiple(9), 2520);
    assert_eq!(least_common_multiple(10), 2520);
}
