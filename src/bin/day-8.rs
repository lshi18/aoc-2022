/*
 * https://adventofcode.com/2022/day/8
 *
 */
fn solve(input: &str) -> (usize, usize) {
    let parsed = input.trim_end().split('\n').collect::<Vec<_>>();
    let col_cnt = parsed.first().unwrap().len();
    let row_cnt = parsed.len();

    let mut visibility_result = vec![(false, 1); row_cnt * col_cnt];

    let mut map = Vec::new();
    let mut transposed_map = Vec::new();

    for (i, &s) in parsed.iter().enumerate() {
        let mut row = Vec::new();
        for (j, c) in s.char_indices() {
            row.push(c.to_digit(10).unwrap() as usize);

            if i == 0 {
                let col = Vec::new();
                transposed_map.push(col)
            }

            transposed_map[j].push(c.to_digit(10).unwrap() as usize);
        }
        map.push(row);
    }

    check_visibility(&map, &mut visibility_result, |i, j| i * col_cnt + j);
    check_visibility(&transposed_map, &mut visibility_result, |j, i| {
        i * col_cnt + j
    });
    (
        visibility_result.iter().filter(|(vis, _)| *vis).count(),
        visibility_result.iter().map(|(_, sc)| *sc).max().unwrap(),
    )
}

fn check_visibility<F>(map: &Vec<Vec<usize>>, result: &mut Vec<(bool, usize)>, index_fn: F)
where
    F: Fn(usize, usize) -> usize,
{
    // print!("{:?}\n", map);
    for (i, row) in map.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            // print!("{}, {}, {}\n", i, j, elem);
            let (left, right) = row.split_at(j);

            // rev() so that looking from right to left
            match left.iter().rev().position(|c| c >= elem) {
                None => {
                    result[index_fn(i, j)].0 |= true;
                    result[index_fn(i, j)].1 *= j;
                }
                Some(jl) => {
                    result[index_fn(i, j)].1 *= jl + 1;
                }
            };
            match right[1..].iter().position(|c| c >= elem) {
                None => {
                    result[index_fn(i, j)].0 |= true;
                    result[index_fn(i, j)].1 *= right[1..].len();
                }
                Some(jr) => {
                    result[index_fn(i, j)].1 *= jr + 1;
                }
            };
            //            println!("i: {}, j: {}, res: {:?}", i, j, result[index_fn(i, j)]);
        }
    }
}

fn main() {
    println!("Run test with `cargo test --package aoc-2022 --bin day-8 --`!");
}

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn test_with_sample() {
        let input = "30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390";

        assert_eq!((21, 8), super::solve(input));
    }

    #[test]
    fn test_with_input() {
        let input = String::from_utf8(fs::read("input-day8.txt").unwrap()).unwrap();
        assert_eq!((1703, 496650), super::solve(&input));
    }
}
