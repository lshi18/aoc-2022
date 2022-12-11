/*
 * Problem: https://adventofcode.com/2022/day/6
*/
use std::fs;
fn main() {
    let input = String::from_utf8(fs::read("input-day6.txt").unwrap()).unwrap();

    let s = input.chars().collect::<Vec<_>>();

    let res1 = find_marker(&s, 4).and_then(|x| Some(x + 1)).unwrap();
    assert!(res1 == 1578, "part 1: expect: 1578; got: {:?}", res1);

    let res2 = find_marker(&s, 14).and_then(|x| Some(x + 1)).unwrap();
    assert!(res2 == 2178, "part 2: expect: 2178; got: {:?}", res2);
}

fn find_marker(s: &[char], size: usize) -> Option<usize> {
    // i is the window start index. Window size is of `size`.
    let mut w_start = 0;
    let mut w_end = 0;

    while w_end < s.len() {
        w_end += 1;

        if let Some(k) = find_dup_index(&s[w_start..w_end], &s[w_end]) {
            w_start = w_start + k + 1;
        } else if w_end - w_start == size - 1 {
            return Some(w_end);
        }
    }
    // none detected successfully.
    None
}

fn find_dup_index(s: &[char], ch: &char) -> Option<usize> {
    for (idx, c) in s.iter().enumerate() {
        if c == ch {
            return Some(idx);
        }
    }
    None
}
