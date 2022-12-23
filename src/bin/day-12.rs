/*
 * https://adventofcode.com/2022/day/12
 */
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add_to_pos(&self, dir: (i32, i32)) -> Pos {
        Pos {
            x: self.x + dir.0,
            y: self.y + dir.1,
        }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Map(HashMap<Pos, char>);

impl Map {
    fn new() -> Self {
        Self(HashMap::<Pos, char>::new())
    }

    fn valid_moves(&self, (curr_pos, curr_val): &(Pos, char)) -> Vec<(Pos, char)> {
        let mut neighbors = vec![];

        for p in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = curr_pos.add_to_pos(p);
            if let Some(neighbor) = self.0.get(&new_pos).and_then(|ch| {
                let c = if *ch == 'E' { 'z' } else { *ch };

                if c as u8 <= *curr_val as u8 + 1 {
                    Some((new_pos, *ch))
                } else {
                    None
                }
            }) {
                neighbors.push(neighbor);
            }
        }
        // println!("neighbors for {:?}: {:?}", curr_pos, curr_val);
        neighbors
    }

    fn all_start_points(&self, all_start_set: &mut HashSet<(Pos, char)>, current: (Pos, char)) {
        for neighbor in self.valid_moves(&current) {
            // println!("{:?}", neighbor);
            if all_start_set.contains(&neighbor) {
                continue;
            } else if neighbor.1 == 'b' {
                all_start_set.insert(neighbor);
            } else {
                // another 'a'
                all_start_set.insert(current.clone());
                self.all_start_points(all_start_set, neighbor);
            }
        }
    }
}
fn main() {
    let input = String::from_utf8(fs::read("input-day12.txt").unwrap()).unwrap();
    // let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let mut map = Map::new();

    for (n_row, line) in input.lines().enumerate() {
        for (n_col, c) in line.chars().enumerate() {
            map.0.insert(
                Pos {
                    x: n_col as i32,
                    y: n_row as i32,
                },
                c,
            );
        }
    }
    let mut result = BTreeSet::<Pos>::new();

    let mut min_path_for_pos = HashMap::<Pos, usize>::new();

    part_1(&map, &mut result, &mut min_path_for_pos);

    part_2(&map, &mut result, &mut min_path_for_pos)
}
fn part_1(map: &Map, result: &mut BTreeSet<Pos>, min_path_for_pos: &mut HashMap<Pos, usize>) {
    let (start, _) = map.0.iter().filter(|(_k, v)| **v == 'S').nth(0).unwrap();

    search(
        &map,
        &(start.clone(), 'a'),
        BTreeSet::<Pos>::new(),
        result,
        min_path_for_pos,
    );

    println!("{:?}", min_path_for_pos);
    println!("Part 1 result:\n{:?}", result.len() - 1);
}

fn part_2(map: &Map, result: &mut BTreeSet<Pos>, min_path_for_pos: &mut HashMap<Pos, usize>) {
    let (start_pos, c) = map.0.iter().filter(|(_k, v)| **v == 'S').nth(0).unwrap();
    let mut start_set = HashSet::<(Pos, char)>::new();

    let start = (start_pos.clone(), 'a');

    map.all_start_points(&mut start_set, start);

    for s in start_set.into_iter().filter(|(_, c)| *c == 'b') {
        println!("start_position: {:?}", s);
        search(&map, &s, BTreeSet::<Pos>::new(), result, min_path_for_pos);
    }
    println!("Part 2 result:\n{:?}", result.len());
}

fn search(
    map: &Map,
    current: &(Pos, char),
    mut visited: BTreeSet<Pos>,
    result: &mut BTreeSet<Pos>,
    min_path_for_pos: &mut HashMap<Pos, usize>,
) {
    if let Some(min) = min_path_for_pos.get_mut(&current.0) {
        if *min > visited.len() {
            *min = visited.len();
        } else {
            // print!(".");
            return;
        }
    } else {
        min_path_for_pos.insert(current.0.clone(), visited.len());
    }

    // println!("{:?}, visited: {:?}", current, visited);
    if !result.is_empty() && visited.len() >= result.len() - 1 {
        return;
    }

    visited.insert(current.0.clone());
    if current.1 == 'E' {
        // println!("End");
        *result = visited;
        return;
    }

    for next in map.valid_moves(current).iter() {
        if visited.contains(&next.0) {
            continue;
        };

        search(map, next, visited.clone(), result, min_path_for_pos);
    }
}
