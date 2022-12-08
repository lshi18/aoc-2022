/*
 * https://adventofcode.com/2022/day/3
 */
use std::fs;

fn main() {
    let input = String::from_utf8(fs::read("input-day3.txt").unwrap()).unwrap();

    let r1 = part1(&input);
    let r2 = part2(&input);

    assert!(r1 == 7831, "part 1 res error {}", r1);
    assert!(r2 == 2683, "part 2 res error {}", r2);
}

fn part1(input: &str) -> u32 {
    input
        .trim_end()
        .split('\n')
        .flat_map(|l| {
            let (f, s) = l.split_at(l.len() / 2);
            overlapping_chars(&[f.as_bytes(), s.as_bytes()])
        })
        .map(|x| match x as char {
            'a'..='z' => x as u32 - 'a' as u32 + 1,
            'A'..='Z' => x as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        })
        //.collect::<Vec<_>>();
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let strings = input.trim_end().split("\n").collect::<Vec<_>>();
    strings
        .chunks_exact(3)
        .flat_map(|s| {
            assert!(s.len() == 3);
            let bs = s.iter().map(|&ss| ss.as_bytes()).collect::<Vec<_>>();
            overlapping_chars(&bs)
        })
        .map(|x| match x as char {
            'a'..='z' => x as u32 - 'a' as u32 + 1,
            'A'..='Z' => x as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        })
        //.collect::<Vec<_>>();
        .sum::<u32>()
}

// fn overlapping_chars(s1: &[u8], s2: &[u8]) -> Vec<u8> {
//     let mut res = Vec::new();
//     for &c in s1 {
//         if s2.contains(&c) {
//             res.push(c);
//         }
//     }
//     res.dedup();
//     res
// }

fn overlapping_chars(ss: &[&[u8]]) -> Vec<u8> {
    let first = ss[0].to_vec();
    let rest = &ss[1..];

    let mut res = rest.iter().fold(first, |mut acc, &x| {
        acc.retain(|&c| x.contains(&c));
        acc
    });
    res.dedup();
    res
}
