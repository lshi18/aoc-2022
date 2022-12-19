/*
 * https://adventofcode.com/2022/day/8
 *
 * The code here are not refactored
 *
 * Can it do better for part 2?
 */
use std::collections::{HashMap, VecDeque};
use std::fs;

type TreeHeight = usize;
#[derive(Debug)]
struct Tree {
    position: Position,
    height: TreeHeight,
    visibility: usize,
}

type Position = (usize, usize);

impl Tree {
    fn new(i: usize, j: usize, tree_height: TreeHeight) -> Self {
        Tree {
            position: (i, j),
            height: tree_height,
            visibility: 4,
        }
    }

    fn compare_height_i(
        &mut self,
        max_height_map: &mut HashMap<(char, usize), TreeHeight>,
        visible_cnt: &mut usize,
    ) {
        let (i, _) = self.position;
        match max_height_map.get_mut(&('i', i)) {
            None => {
                max_height_map.insert(('i', i), self.height);
            }
            Some(curr_max) => {
                if curr_max >= &mut self.height {
                    self.visibility -= 1;
                    if self.visibility == 0 {
                        *visible_cnt -= 1;
                    }
                } else {
                    max_height_map.insert(('i', i), self.height);
                }
            }
        }
    }

    fn compare_height_j(
        &mut self,
        max_height_map: &mut HashMap<(char, usize), TreeHeight>,
        visible_cnt: &mut usize,
    ) {
        let (_, j) = self.position;
        match max_height_map.get_mut(&('j', j)) {
            None => {
                max_height_map.insert(('j', j), self.height);
            }
            Some(curr_max) => {
                if curr_max >= &mut self.height {
                    self.visibility -= 1;
                    if self.visibility == 0 {
                        *visible_cnt -= 1;
                    }
                } else {
                    max_height_map.insert(('j', j), self.height);
                }
            }
        }
    }
}

fn calc_scenic_scores(row: &VecDeque<Tree>, scores: &mut HashMap<Position, usize>) {
    let length = row.len();

    for (idx, tree) in row.iter().enumerate() {
        let prefix = row.range(0..idx);
        let suffix = row.range(idx + 1..length);

        let p_v = prefix
            .filter(|&t| t.height >= tree.height)
            .last()
            .and_then(|x| Some(idx - x.position.1))
            .or(Some(idx))
            .unwrap();

        let s_v = suffix
            .filter(|&t| t.height >= tree.height)
            .nth(0)
            .and_then(|x| Some(x.position.1 - idx))
            .or(Some(length - idx - 1))
            .unwrap();

        update_score(scores, tree.position, p_v * s_v);
        // println!(
        //     "scores: {:?}, p_v: {}, s_v {}, pos: {:?}",
        //     scores, p_v, s_v, tree.position
        // );
    }
}

fn calc_scenic_scores2(row: &VecDeque<Tree>, scores: &mut HashMap<Position, usize>) {
    let length = row.len();

    for (idx, tree) in row.iter().enumerate() {
        let prefix = row.range(0..idx);
        let suffix = row.range(idx + 1..length);

        let p_v = prefix
            .filter(|&t| t.height >= tree.height)
            .last()
            .and_then(|x| Some(idx - x.position.1))
            .or(Some(idx))
            .unwrap();

        let s_v = suffix
            .filter(|&t| t.height >= tree.height)
            .nth(0)
            .and_then(|x| Some(x.position.1 - idx))
            .or(Some(length - idx - 1))
            .unwrap();

        update_score(scores, (tree.position.1, tree.position.0), p_v * s_v);
        //  println!(
        //     "scores: {:?}, p_v: {}, s_v {}, pos: {:?}",
        //     scores,
        //     p_v,
        //     s_v,
        //     (tree.position.1, tree.position.0)
        // );
    }
}

fn update_score(scores: &mut HashMap<Position, usize>, pos: Position, score: usize) {
    match scores.get(&pos) {
        None => {
            scores.insert(pos, score);
        }
        Some(v) => {
            scores.insert(pos, score * v);
        }
    }
}

fn main() {
    let input = String::from_utf8(fs::read("input-day8.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let parsed = input.trim_end().split('\n').collect::<Vec<_>>();

    let mut visible_tree_cnt = parsed.iter().map(|&x| x.len()).sum::<usize>();

    // println!("total tree count {:?}", visible_tree_cnt);

    let mut map = VecDeque::new();
    let mut transposed_map = VecDeque::new();

    for (i, &s) in parsed.iter().enumerate() {
        let mut row = VecDeque::new();
        for (j, c) in s.char_indices() {
            row.push_back(Tree::new(i, j, c.to_digit(10).unwrap() as usize));

            if i == 0 {
                let col: VecDeque<Tree> = VecDeque::new();
                transposed_map.push_back(col)
            }

            transposed_map[j].push_back(Tree::new(j, i, c.to_digit(10).unwrap() as usize));
        }
        map.push_back(row);
    }

    // println!("initial map: {:?}", transposed_map);

    let mut max_height_map = HashMap::<(char, usize), usize>::new();

    for r in map.iter_mut() {
        for tree in r.iter_mut() {
            // println!("{:?}", tree);
            tree.compare_height_i(&mut max_height_map, &mut visible_tree_cnt);
            tree.compare_height_j(&mut max_height_map, &mut visible_tree_cnt);
        }
    }

    // !! MUST reset the max_height_map before the reversing traversal.
    let mut max_height_map = HashMap::<(char, usize), usize>::new();

    for r in map.iter_mut().rev() {
        for tree in r.iter_mut().rev() {
            tree.compare_height_i(&mut max_height_map, &mut visible_tree_cnt);
            tree.compare_height_j(&mut max_height_map, &mut visible_tree_cnt);
        }
    }

    let mut scenic_scores = HashMap::<Position, usize>::new();

    for r in map.iter() {
        calc_scenic_scores(&r, &mut scenic_scores);
    }

    for r in transposed_map {
        calc_scenic_scores2(&r, &mut scenic_scores);
    }

    let max_sc = scenic_scores.values().max().unwrap();
    println!("(part 2) max scenic score: {:?}", max_sc);
}

// [[Tree { position: (0, 0), height: 3, visibility: 4 }, Tree { position: (0, 1), height: 0, visibility: 4 }, Tree { position: (0, 2), height: 3, visibility: 4 }, Tree { position: (0, 3), height: 7, visibility: 4 }, Tree { position: (0, 4), height: 3, visibility: 4 }],
//  [Tree { position: (1, 0), height: 2, visibility: 4 }, Tree { position: (1, 1), height: 5, visibility: 4 }, Tree { position: (1, 2), height: 5, visibility: 4 }, Tree { position: (1, 3), height: 1, visibility: 4 }, Tree { position: (1, 4), height: 2, visibility: 4 }],
//  [Tree { position: (2, 0), height: 6, visibility: 4 }, Tree { position: (2, 1), height: 5, visibility: 4 }, Tree { position: (2, 2), height: 3, visibility: 4 }, Tree { position: (2, 3), height: 3, visibility: 4 }, Tree { position: (2, 4), height: 2, visibility: 4 }],
//  [Tree { position: (3, 0), height: 3, visibility: 4 }, Tree { position: (3, 1), height: 3, visibility: 4 }, Tree { position: (3, 2), height: 5, visibility: 4 }, Tree { position: (3, 3), height: 4, visibility: 4 }, Tree { position: (3, 4), height: 9, visibility: 4 }],
//  [Tree { position: (4, 0), height: 3, visibility: 4 }, Tree { position: (4, 1), height: 5, visibility: 4 }, Tree { position: (4, 2), height: 3, visibility: 4 }, Tree { position: (4, 3), height: 9, visibility: 4 }, Tree { position: (4, 4), height: 0, visibility: 4 }]]

// [[Tree { position: (0, 0), height: 3, visibility: 4 }, Tree { position: (1, 0), height: 2, visibility: 4 }, Tree { position: (2, 0), height: 6, visibility: 4 }, Tree { position: (3, 0), height: 3, visibility: 4 }, Tree { position: (4, 0), height: 3, visibility: 4 }], [Tree { position: (0, 1), height: 0, visibility: 4 }, Tree { position: (1, 1), height: 5, visibility: 4 }, Tree { position: (2, 1), height: 5, visibility: 4 }, Tree { position: (3, 1), height: 3, visibility: 4 }, Tree { position: (4, 1), height: 5, visibility: 4 }], [Tree { position: (0, 2), height: 3, visibility: 4 }, Tree { position: (1, 2), height: 5, visibility: 4 }, Tree { position: (2, 2), height: 3, visibility: 4 }, Tree { position: (3, 2), height: 5, visibility: 4 }, Tree { position: (4, 2), height: 3, visibility: 4 }], [Tree { position: (0, 3), height: 7, visibility: 4 }, Tree { position: (1, 3), height: 1, visibility: 4 }, Tree { position: (2, 3), height: 3, visibility: 4 }, Tree { position: (3, 3), height: 4, visibility: 4 }, Tree { position: (4, 3), height: 9, visibility: 4 }], [Tree { position: (0, 4), height: 3, visibility: 4 }, Tree { position: (1, 4), height: 2, visibility: 4 }, Tree { position: (2, 4), height: 2, visibility: 4 }, Tree { position: (3, 4), height: 9, visibility: 4 }, Tree { position: (4, 4), height: 0, visibility: 4 }]]

// {(2, 1): 6, (0, 2): 0, (4, 1): 0, (1, 3): 1, (2, 4): 0, (3, 1): 1, (4, 3): 0, (0, 0): 0, (4, 2): 0, (2, 3): 2, (1, 0): 0, (3, 3): 3, (3, 0): 0, (1, 4): 0, (2, 0): 0, (3, 4): 0, (4, 4): 0, (4, 0): 0, (1, 1): 1, (1, 2): 4, (2, 2): 1, (0, 1): 0, (0, 3): 0, (0, 4): 0, (3, 2): 8}
