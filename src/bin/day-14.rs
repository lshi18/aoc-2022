/*
 * https://adventofcode.com/2022/day/14
 *
 * Correct solution but the code is not very clean!
 */
use std::collections::{HashMap, HashSet};
use std::{fmt, fs};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Unit {
    Air,
    RestSand,
    Sand,
    Entry,
    Rock,
}

enum State {
    NewSand,
    Falling,
    End,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Unit::Air => '.',
            Unit::Entry => '+',
            Unit::Rock => '#',
            Unit::RestSand => 'o',
            Unit::Sand => '*',
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct CaveMap {
    max_x: i32,
    max_y: i32,
    min_x: i32,
    falling_sand_pos: Pos,
    data: HashMap<Pos, Unit>,
}

impl CaveMap {
    fn new(rocks: &HashSet<(i32, i32)>, max_x: i32, max_y: i32, min_x: i32) -> Self {
        let mut data = HashMap::new();

        for x in min_x..=max_x {
            for y in 0..=max_y {
                if rocks.contains(&(x, y)) {
                    data.insert(Pos { x, y }, Unit::Rock);
                }
            }
        }

        data.insert(Pos { x: 500, y: 0 }, Unit::Entry);

        Self {
            max_x,
            max_y,
            min_x,
            data,
            falling_sand_pos: Pos { x: 500, y: 0 },
        }
    }

    fn new_with_floor(rocks: &HashSet<(i32, i32)>, max_x: i32, max_y: i32, min_x: i32) -> Self {
        let mut cave = Self::new(rocks, max_x, max_y, min_x);

        cave.max_y = max_y + 2;
        for x in min_x..=max_x {
            cave.data.insert(Pos { x, y: max_y + 2 }, Unit::Rock);
        }
        cave
    }

    fn tick(&mut self, state: &State) -> State {
        match &state {
            State::NewSand => {
                self.falling_sand_pos = Pos { x: 500, y: 0 };

                State::Falling
            }
            State::Falling => {
                if let Some(pos) = self.next_pos() {
                    let prev_pos = &self.falling_sand_pos;
                    if !(prev_pos.x == 500 && prev_pos.y == 0) {
                        self.data.remove(&prev_pos);
                    }
                    self.data.insert(pos.clone(), Unit::Sand);
                    self.falling_sand_pos = pos.clone();

                    if pos.x == self.min_x || pos.x == self.max_x || pos.y == self.max_y {
                        State::End
                    } else {
                        State::Falling
                    }
                } else {
                    let to_rest = &self.falling_sand_pos;
                    self.data.insert(to_rest.clone(), Unit::RestSand);

                    State::NewSand
                }
            }
            State::End => State::End,
        }
    }

    fn rest_sand_count(&self) -> usize {
        self.data
            .iter()
            .filter(|&(_, unit)| {
                if let Unit::RestSand = unit {
                    true
                } else {
                    false
                }
            })
            .count()
    }

    fn next_pos(&self) -> Option<Pos> {
        let Pos { x, y } = self.falling_sand_pos;

        let next = vec![
            Pos { x, y: y + 1 },
            Pos { x: x - 1, y: y + 1 },
            Pos { x: x + 1, y: y + 1 },
        ];

        let mut ret = None;

        for n in next {
            if let Some(unit) = self.data.get(&n) {
                match unit {
                    Unit::Rock | Unit::RestSand => continue,
                    _ => unreachable!(),
                }
            } else {
                ret = Some(n);
                break;
            }
        }
        ret
    }
}

impl fmt::Display for CaveMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();

        for y in 0..=self.max_y {
            str.push_str(format!("{0: <5}:  ", y).as_str());
            for x in self.min_x..=self.max_x {
                let s = format!(
                    "{}",
                    self.data
                        .get(&Pos { x, y })
                        .or_else(|| Some(&Unit::Air))
                        .unwrap()
                );
                str.push_str(&s);
            }
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

impl CaveMap {
    fn tick_with_floor(&mut self, state: &State) -> State {
        match &state {
            State::NewSand => {
                self.falling_sand_pos = Pos { x: 500, y: 0 };

                State::Falling
            }
            State::Falling => {
                if let Some(pos) = self.next_pos_with_floor() {
                    let prev_pos = &self.falling_sand_pos;
                    if !(prev_pos.x == 500 && prev_pos.y == 0) {
                        self.data.remove(&prev_pos);
                    }
                    self.data.insert(pos.clone(), Unit::Sand);
                    self.falling_sand_pos = pos.clone();

                    State::Falling
                } else {
                    if self.falling_sand_pos.x == 500 && self.falling_sand_pos.y == 0 {
                        return State::End;
                    }
                    let to_rest = &self.falling_sand_pos;
                    self.data.insert(to_rest.clone(), Unit::RestSand);

                    State::NewSand
                }
            }
            State::End => State::End,
        }
    }

    fn next_pos_with_floor(&mut self) -> Option<Pos> {
        let Pos { x, y } = self.falling_sand_pos;

        let next = vec![
            Pos { x, y: y + 1 },
            Pos { x: x - 1, y: y + 1 },
            Pos { x: x + 1, y: y + 1 },
        ];

        let mut ret = None;

        for n in next {
            if let Some(unit) = self.data.get(&n) {
                match unit {
                    Unit::Rock | Unit::RestSand => continue,
                    _ => unreachable!(),
                }
            } else {
                if n.x < self.min_x {
                    self.min_x = n.x;
                }
                if n.x > self.max_x {
                    self.max_x = n.x;
                }
                if n.y == self.max_y {
                    self.data.insert(Pos { x: n.x, y: n.y }, Unit::Rock);
                    continue;
                }
                ret = Some(Pos { x: n.x, y: n.y });
                break;
            }
        }
        ret
    }
}

fn main() {
    let input = String::from_utf8(fs::read("input-day14.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();
    let mut lines_points = HashSet::<(i32, i32)>::new();

    let (mut min_x, mut max_x, mut max_y) = (i32::MAX, i32::MIN, i32::MIN);

    for l in input.trim_end().lines() {
        let line = l
            .split(" -> ")
            .map(|s| {
                if let Some((x, y)) = s.split_once(',') {
                    let x = x.parse::<i32>().unwrap();
                    let y = y.parse::<i32>().unwrap();
                    if x > max_x {
                        max_x = x;
                    }
                    if x < min_x {
                        min_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }

                    (x, y)
                } else {
                    unreachable!()
                }
            })
            .collect::<Vec<_>>();

        let mut head = &line[0];
        let mut rest = &line[1..];
        while !rest.is_empty() {
            lines_points.insert(head.clone());
            let &(x, y) = &rest[0];
            if head.0 == x && head.1 >= y {
                for j in y..=head.1 {
                    lines_points.insert((x, j));
                }
            }

            if head.0 == x && head.1 < y {
                for j in head.1..=y {
                    lines_points.insert((x, j));
                }
            }

            if head.1 == y && head.0 >= x {
                for i in x..=head.0 {
                    lines_points.insert((i, y));
                }
            }

            if head.1 == y && head.0 < x {
                for i in head.0..=x {
                    lines_points.insert((i, y));
                }
            }

            head = &rest[0];
            rest = &rest[1..];
        }
    }

    part_1(&lines_points, &(min_x, max_y, max_x));
    part_2(&lines_points, &(min_x, max_y, max_x));
}

fn part_2(rocks: &HashSet<(i32, i32)>, &(min_x, max_y, max_x): &(i32, i32, i32)) {
    let mut cave = CaveMap::new_with_floor(rocks, max_x, max_y, min_x);
    // println!("{}", cave);
    let mut state = cave.tick(&State::NewSand);

    loop {
        match cave.tick_with_floor(&state) {
            State::End => {
                //println!("Part 2 End!\n{}", cave);
                break;
            }
            next_state => {
                // println!("{}", cave);
                state = next_state;
            }
        }
    }
    let result = cave.rest_sand_count() + 1;
    // assert!(result == 93, "sample real: {}", result);
    assert!(result == 27623, "real: {}", result);
}

fn part_1(rocks: &HashSet<(i32, i32)>, &(min_x, max_y, max_x): &(i32, i32, i32)) {
    let mut cave = CaveMap::new(rocks, max_x, max_y, min_x);
    // println!("{}", cave);

    let mut state = cave.tick(&State::NewSand);

    loop {
        match cave.tick(&state) {
            State::End => {
                // println!("Part 1 End!\n{}", cave);
                break;
            }
            next_state => {
                // println!("{}", cave);
                state = next_state;
            }
        }
    }
    let result = cave.rest_sand_count();
    // assert!(result == 24, "sample real: {}", result);
    assert!(result == 728, "real: {}", result);
}
