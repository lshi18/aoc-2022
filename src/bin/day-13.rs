/*
 * https://adventofcode.com/2022/day/13
 */

use serde_json::Value;
use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = String::from_utf8(fs::read("input-day13.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    part_1(&input);

    part_2(&input);
}

fn part_1(input: &str) {
    let mut acc: usize = 0;

    for (i, pairs) in input.trim_end().split("\n\n").enumerate() {
        let lists = pairs
            .lines()
            .map(|d| serde_json::from_str::<Value>(d).unwrap())
            .collect::<Vec<_>>();

        // println!("{:?}", lists);
        // println!("0: {:?}\n1: {:?}", lists[0], lists[1]);
        let res = cmp(&lists[0], &lists[1]);

        if let Ordering::Less = res {
            acc += i + 1;
        }
    }
    println!("part 1 result: {}", acc);
}

fn part_2(input: &str) {
    let mut v = input
        .trim_end()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str::<Value>(s).unwrap())
        .collect::<Vec<_>>();

    let divider_pkgs = vec![
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];
    let mut cloned = divider_pkgs.clone();

    v.append(&mut cloned);
    v.sort_by(|a, b| cmp(a, b));

    let mut res = 1usize;
    for (i, x) in v.iter().enumerate() {
        if divider_pkgs.contains(x) {
            res *= i + 1;
        }
    }

    println!("part 2 result: {}", res);
}

fn cmp(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Array(l_array), Value::Array(r_array)) => cmp_arrays(l_array, r_array),
        (Value::Array(_l_array), Value::Number(num)) => {
            cmp(left, &Value::Array(vec![Value::Number(num.clone())]))
        }
        (Value::Number(num), Value::Array(_r_array)) => {
            cmp(right, &Value::Array(vec![Value::Number(num.clone())])).reverse()
        }
        (Value::Number(l_num), Value::Number(r_num)) => {
            l_num.as_i64().unwrap().cmp(&r_num.as_i64().unwrap())
        }

        _ => unreachable!(),
    }
}

fn cmp_arrays(arr1: &[Value], arr2: &[Value]) -> Ordering {
    match (arr1.is_empty(), arr2.is_empty()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => match cmp(&arr1[0], &arr2[0]) {
            Ordering::Equal => cmp_arrays(&arr1[1..], &arr2[1..]),
            ord => ord,
        },
    }
}
