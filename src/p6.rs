pub fn solve() -> usize {
    return difference_of_square_of_sum_and_sum_of_squares(100);
}

pub fn difference_of_square_of_sum_and_sum_of_squares(n: usize) -> usize {
    let s = sum(n);
    return s * s - sum_of_squares(n);
}

pub fn sum(n: usize) -> usize {
    return n * (n + 1) / 2;
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut s = 0;
    for i in 1..n+1 {
        s += i * i;
    }
    return s;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 25_164_150);
}

#[test]
fn test_difference_of_square_of_sum_and_sum_of_squares() {
    assert_eq!(difference_of_square_of_sum_and_sum_of_squares(10), 2640);
}

#[test]
fn test_sum() {
    assert_eq!(sum(0), 0);
    assert_eq!(sum(1), 1);
    assert_eq!(sum(2), 3);
    assert_eq!(sum(3), 6);
    assert_eq!(sum(4), 10);
    assert_eq!(sum(5), 15);
    assert_eq!(sum(6), 21);
    assert_eq!(sum(7), 28);
    assert_eq!(sum(8), 36);
    assert_eq!(sum(9), 45);
    assert_eq!(sum(10), 55);
}

#[test]
fn test_sum_of_squares() {
    assert_eq!(sum_of_squares(0), 0);
    assert_eq!(sum_of_squares(1), 1);
    assert_eq!(sum_of_squares(2), 5);
    assert_eq!(sum_of_squares(3), 14);
    assert_eq!(sum_of_squares(4), 30);
    assert_eq!(sum_of_squares(5), 55);
    assert_eq!(sum_of_squares(6), 91);
    assert_eq!(sum_of_squares(7), 140);
    assert_eq!(sum_of_squares(8), 204);
    assert_eq!(sum_of_squares(9), 285);
    assert_eq!(sum_of_squares(10), 385);
}
