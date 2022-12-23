/*
 * https://adventofcode.com/2022/day/11
 */
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    operation: (Op, Val),
    test: (u64, u64, u64),
    inspections: u64,
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Val {
    Old,
    Num(u64),
}

impl Monkey {
    fn from_string(s: &str) -> Self {
        if let [id, start_items, operation, test, if_true, if_false] =
            s.split('\n').collect::<Vec<_>>()[..]
        {
            return Self {
                id: parse_id(id),
                items: parse_start_items(start_items),
                operation: parse_operation(operation),
                test: parse_test(test, if_true, if_false),
                inspections: 0,
            };
        }
        unreachable!()
    }

    fn process_items<F>(&mut self, thrown: &mut HashMap<u64, VecDeque<u64>>, f: F)
    where
        F: Fn(u64) -> u64,
    {
        if let Some(mut items) = thrown.remove(&self.id) {
            self.items.append(&mut items);
        }

        while let Some(item) = self.items.pop_front() {
            self.inspections += 1;
            let worry_level = apply_operation(&self.operation, item);
            let worry_level = f(worry_level);

            let (divisor, if_true, if_false) = &self.test;

            if worry_level % divisor == 0 {
                throw_to_monkey(worry_level, *if_true, thrown);
            } else {
                throw_to_monkey(worry_level, *if_false, thrown);
            }
        }
    }
}

fn part1(input: &str) {
    let mut monkeys = input
        .trim_end()
        .split("\n\n")
        .map(|x| Monkey::from_string(x))
        .collect::<Vec<_>>();

    // println!("{:?}", input);
    let mut thrown = HashMap::<u64, VecDeque<u64>>::new();

    for _i in 1..=10000 {
        for monkey in monkeys.iter_mut() {
            monkey.process_items(&mut thrown, |wl| wl / 3);
        }
    }

    let mut answer = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    answer.sort();
    //println!("thrown: {:?}", thrown);
    println!(
        "Part 1 answer: {:?}, first 2: {:?}, all: {:?}",
        answer.iter().rev().take(2).fold(1, |acc, x| acc * x),
        answer.iter().rev().take(2).collect::<Vec<_>>(),
        answer
    );
}

fn part2(input: &str) {
    let mut monkeys = input
        .trim_end()
        .split("\n\n")
        .map(|x| Monkey::from_string(x))
        .collect::<Vec<_>>();

    let mut thrown = HashMap::<u64, VecDeque<u64>>::new();
    let management_index = monkeys.iter().map(|m| m.test.0).fold(1, |acc, x| acc * x);

    for _i in 1..=10000 {
        for monkey in monkeys.iter_mut() {
            monkey.process_items(&mut thrown, |wl| wl % management_index);
        }
    }

    let mut answer = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    answer.sort();
    //println!("thrown: {:?}", thrown);
    println!(
        "Part 2 answer: {:?}, first 2: {:?}, all: {:?}",
        answer.iter().rev().take(2).fold(1, |acc, x| acc * x),
        answer.iter().rev().take(2).collect::<Vec<_>>(),
        answer
    );
}

fn main() {
    let input = String::from_utf8(fs::read("input-day11.txt").unwrap()).unwrap();
    part1(&input);
    part2(&input);
}

fn throw_to_monkey(item: u64, id: u64, thrown: &mut HashMap<u64, VecDeque<u64>>) {
    if let Some(items) = thrown.get_mut(&id) {
        items.push_back(item);
    } else {
        let mut items = VecDeque::new();
        items.push_back(item);
        thrown.insert(id, items);
    }
    // println!("t: {:?}", thrown);
}

fn apply_operation((operator, val): &(Op, Val), operand: u64) -> u64 {
    let operand_2 = match val {
        Val::Old => operand,
        Val::Num(x) => *x,
    };

    match operator {
        Op::Add => operand + operand_2,
        Op::Mul => operand * operand_2,
        Op::Sub => operand - operand_2,
        Op::Div => operand / operand_2,
    }
}

fn parse_id(id: &str) -> u64 {
    let re = Regex::new(r"[\d]+").unwrap();
    let cap = &re.captures_iter(id).next().unwrap()[0];

    cap.parse().unwrap()
}

fn parse_start_items(start_items: &str) -> VecDeque<u64> {
    let mut ret = VecDeque::new();
    let re = Regex::new(r"\d{2}").unwrap();

    for s in re.captures_iter(start_items) {
        ret.push_back(s[0].parse::<u64>().unwrap());
    }

    ret
}

fn parse_operation(op: &str) -> (Op, Val) {
    let re = Regex::new(r"new = old (\*|\+|/|\-) (old|[\d]{1,2})").unwrap();

    let cap = re.captures_iter(op).next().unwrap();
    let operator = match &cap[1] {
        "*" => Op::Mul,
        "+" => Op::Add,
        "-" => Op::Sub,
        "/" => Op::Div,
        _ => unreachable!(),
    };

    let operand = match &cap[2] {
        "old" => Val::Old,
        num => Val::Num(num.parse::<u64>().unwrap()),
    };

    (operator, operand)
}

fn parse_test(test: &str, if_true: &str, if_false: &str) -> (u64, u64, u64) {
    let re_test = Regex::new(r"by ([\d]+)").unwrap();
    let re_to = Regex::new(r"to monkey ([\d]+)").unwrap();

    let divisor = &re_test.captures_iter(test).next().unwrap()[1]
        .parse::<u64>()
        .unwrap();
    let to_true = &re_to.captures_iter(if_true).next().unwrap()[1]
        .parse::<u64>()
        .unwrap();
    let to_false = &re_to.captures_iter(if_false).next().unwrap()[1]
        .parse::<u64>()
        .unwrap();

    (*divisor, *to_true, *to_false)
}
