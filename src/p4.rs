pub fn solve() -> i64 {
    return largest_palindrome_product(3);
}

pub fn largest_palindrome_product(n: i16) -> i64 {
    if n < 1 {
        panic!("invalid input for largest_palindrome_product: {}", n);
    }
    let mut max_str = String::new();
    for _ in 0..n {
        max_str.push('9');
    }
    let max = max_str.parse::<i64>().unwrap();
    let mut min_str = "1".to_string();
    for _ in 0..n-1 {
        min_str.push('0');
    }
    let min = min_str.parse::<i64>().unwrap();
    let mut result = 0;
    for i in (min..max + 1).rev() {
        for j in (min..max + 1).rev() {
            let p = i * j;
            if is_palindrome(p) && p > result {
                result = p;
            }
        }
    }
    return result;
}

pub fn is_palindrome(n: i64) -> bool {
    let str = n.to_string();
    let bytes = str.as_bytes();
    let len = bytes.len();
    let mid = len / 2;
    for i in 0..mid {
        if bytes[i] != bytes[len - i - 1] {
            return false;
        }
    }
    return true;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 906609);
}

#[test]
fn test_largest_palindrome_product() {
    assert_eq!(largest_palindrome_product(2), 9009);
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(9009));
}
