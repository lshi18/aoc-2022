use std::fs;

fn main() {
    let input = String::from_utf8(fs::read("input-day3.txt").unwrap()).unwrap();

    part1(&input);

    part2(&input);
}

fn part1(input: &str) {
    let res = input
        .trim_end()
        .split('\n')
        .flat_map(|l| {
            let (f, s) = l.split_at(l.len() / 2);
            overlapping_chars(f.as_bytes(), s.as_bytes())
        })
        .map(|x| match x as char {
            'a'..='z' => x as u32 - 'a' as u32 + 1,
            'A'..='Z' => x as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        })
        //.collect::<Vec<_>>();
        .sum::<u32>();
    println!("Part 1 result: {}", res);
}

fn part2(input: &str) {
    let strings = input.trim_end().split("\n").collect::<Vec<_>>();
    let res = strings
        .chunks_exact(3)
        .flat_map(|s| {
            assert!(s.len() == 3);
            let a = s[0];
            let b = s[1];
            let c = s[2];
            let o = overlapping_chars(a.as_bytes(), b.as_bytes());
            overlapping_chars(&o, c.as_bytes())
        })
        .map(|x| match x as char {
            'a'..='z' => x as u32 - 'a' as u32 + 1,
            'A'..='Z' => x as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        })
        //.collect::<Vec<_>>();
        .sum::<u32>();

    println!("Part 2 result: {:?}", res);
}

fn overlapping_chars(s1: &[u8], s2: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    for &c in s1 {
        if s2.contains(&c) {
            res.push(c);
        }
    }
    res.dedup();
    res
}
