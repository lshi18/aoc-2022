use std::{fs, str};

fn main() {
    let input = String::from_utf8(fs::read("input-day5.txt").unwrap()).unwrap();

    let res = input.trim_end().split("\n\n").collect::<Vec<_>>();
    // let steps = res[1];

    let stacks: [Vec<char>; 9] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    let moves: Vec<(u32, usize, usize)> = vec![];

    let stacks = res[0].split('\n').rfold(stacks, |mut acc, x| {
        let x = x.chars().collect::<Vec<_>>();
        for i in 0..9 {
            let v = x[i * 4 + 1];
            if v != ' ' {
                acc[i].push(v);
            }
        }
        acc
    });

    // println!("1: {:?}", stacks);
    let instrs = res[1].split('\n').fold(moves, |mut acc, s| {
        let s = s.split_ascii_whitespace().collect::<Vec<_>>();
        acc.push((
            s[1].parse::<u32>().unwrap(),
            s[3].parse::<usize>().unwrap(),
            s[5].parse::<usize>().unwrap(),
        ));
        acc
    });

    //println!("2: {:?}", instrs);
    let stacks = instrs.iter().fold(stacks, |mut acc, (cnt, from, to)| {
        for _i in 0..*cnt {
            // Not particularly nice !!!
            let v = acc[*from - 1].pop().unwrap();
            acc[*to - 1].push(v);
        }
        acc
    });
    //println!("3: {:?}", stacks);

    let res = stacks
        .iter()
        .map(|x| *x.last().unwrap() as u8)
        .collect::<Vec<_>>();
    let res = str::from_utf8(&res).unwrap();
    println!("4: {:?}", res);
}
