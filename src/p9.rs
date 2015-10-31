pub fn solve() -> Result<[usize; 3], String> {
    return pythagorean_triplet(1000);
}

fn pythagorean_triplet(sum: usize) -> Result<[usize; 3], String> {
    for c in 0 .. sum {
        for b in 0 .. c {
            for a in 0 ..b {
                if (a + b + c == sum) && (a*a + b*b == c*c) {
                    return Ok([a, b, c]);
                }
            }
        }
    }
    return Err(format!("no triple found summing to {}", sum));
}

#[test]
fn test_solve() {
    assert_eq!(pythagorean_triplet(1000), Ok([200, 375, 425]));
}

#[test]
fn test_pythagorean_triplet() {
    assert_eq!(pythagorean_triplet(12), Ok([3, 4, 5]));
}
