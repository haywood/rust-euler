use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub type Matrix<'a> = Vec<Vec<i64>>;

pub fn solve() -> i64 {
    let file = File::open("p11.txt").unwrap();
    let reader = BufReader::new(file);
    let mut m = Vec::new();
    for line in reader.lines() {
        let mut row = Vec::new();
        for num in line.unwrap().split(" ") {
            row.push(num.parse::<i64>().unwrap());
        }
        m.push(row);
    }
    return greatest_product(&m, 4);
}

pub fn greatest_product(m: &Matrix, n: usize) -> i64 {
    let mut max: i64 = i64::min_value();
    for i in 0 .. 1+m.len()-n {
        for j in 0 .. 1+m.len()-n {
            max = cmp::max(max, compute_product(m, n, i as i64, j as i64, 1, 0));
            max = cmp::max(max, compute_product(m, n, i as i64, j as i64, 1, 1));
            max = cmp::max(max, compute_product(m, n, i as i64, j as i64, 0, 1));
            max = cmp::max(max, compute_product(m, n, i as i64, (m.len() - j - 1) as i64, 1, -1));
        }
    }
    return max;
}

fn compute_product(m: &Matrix, n: usize, mut i: i64, mut j: i64, di: i64, dj: i64) -> i64 {
    let mut p = 1;
    for _ in 0..n {
        p *= m[i as usize][j as usize];
        i += di;
        j += dj;
    }
    return p;
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 70_600_674);
}

#[test]
fn test_compute_product() {
    assert_eq!(
        greatest_product(&vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            ], 4),
        24);
    assert_eq!(
        greatest_product(&vec![
            vec![0, 0, 0, 4],
            vec![0, 0, 3, 0],
            vec![0, 2, 0, 0],
            vec![1, 0, 0, 0],
            ], 4),
        24);
    assert_eq!(
        greatest_product(&vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            ], 4),
        3024);
    assert_eq!(
        greatest_product(&vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            ], 7),
        181440);
}
