use std::fs;

fn main() {
    let (stacks, instrs) = init();

    let part1 = |mut acc: Vec<Vec<char>>, &(cnt, from, to): &(usize, usize, usize)| {
        for _i in 0..cnt {
            // Not particularly nice !!!
            let v = acc[from - 1].pop().unwrap();
            acc[to - 1].push(v);
        }
        acc
    };

    let res1 = solution(stacks.clone(), &instrs, part1);
    assert!(res1 == "QMBMJDFTD", "expected QMBMJDFTD, got {}", res1);

    let part2 = |mut acc: Vec<Vec<char>>, &(cnt, from, to): &(usize, usize, usize)| {
        let mut tmp: Vec<char> = vec![];
        for _i in 0..cnt {
            // Not particularly nice !!!
            let v = acc[from - 1].pop().unwrap();
            tmp.insert(0, v);
        }
        acc[to - 1].append(&mut tmp);
        acc
    };

    let res2 = solution(stacks, &instrs, part2);
    assert!(res2 == "NBTVTJNFJ", "expected NBTVTJNFJ, got {}", res2)
}

fn init() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = String::from_utf8(fs::read("input-day5.txt").unwrap()).unwrap();

    let res = input.trim_end().split("\n\n").collect::<Vec<_>>();
    // let steps = res[1];

    let stacks = vec![
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
    let moves: Vec<(usize, usize, usize)> = vec![];

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
            s[1].parse::<usize>().unwrap(),
            s[3].parse::<usize>().unwrap(),
            s[5].parse::<usize>().unwrap(),
        ));
        acc
    });

    (stacks, instrs)
}

fn solution<F>(stacks: Vec<Vec<char>>, instrs: &Vec<(usize, usize, usize)>, f: F) -> String
where
    F: FnMut(Vec<Vec<char>>, &(usize, usize, usize)) -> Vec<Vec<char>>,
{
    //println!("2: {:?}", instrs);
    let updated_stacks = instrs.iter().fold(stacks, f);
    // println!("3: {:?}", stacks);

    let res = updated_stacks
        .into_iter()
        .map(|x| *x.last().unwrap() as u8)
        .collect::<Vec<_>>();
    String::from_utf8(res).unwrap()
}
