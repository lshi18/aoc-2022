use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug)]
struct Tree {
    position: (usize, usize),
    height: usize,
    visibility: usize,
}

impl Tree {
    fn new(i: usize, j: usize, tree_height: usize) -> Self {
        Tree {
            position: (i, j),
            height: tree_height,
            visibility: 4,
        }
    }

    fn compare_height_i(
        &mut self,
        max_height_map: &mut HashMap<(char, usize), usize>,
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
        max_height_map: &mut HashMap<(char, usize), usize>,
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

fn main() {
    let input = String::from_utf8(fs::read("input-day8.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let parsed = input.trim_end().split('\n').collect::<Vec<_>>();

    let mut visible_tree_cnt = parsed.iter().map(|&x| x.len()).sum::<usize>();

    println!("total tree count {:?}", visible_tree_cnt);

    let mut map = VecDeque::new();

    for (i, &s) in parsed.iter().enumerate() {
        let mut row = VecDeque::new();
        for (j, c) in s.char_indices() {
            row.push_back(Tree::new(i, j, c.to_digit(10).unwrap() as usize));
        }
        map.push_back(row);
    }

    // println!("initial map: {:?}", map);

    let mut max_height_map = HashMap::<(char, usize), usize>::new();

    // println!("normal order start");
    for r in map.iter_mut() {
        for tree in r.iter_mut() {
            // println!("{:?}", tree);
            tree.compare_height_i(&mut max_height_map, &mut visible_tree_cnt);
            tree.compare_height_j(&mut max_height_map, &mut visible_tree_cnt);
        }
    }

    // !! MUST reset the max_height_map before the reversing traversal.
    let mut max_height_map = HashMap::<(char, usize), usize>::new();
    // println!("reverse order start");
    for r in map.iter_mut().rev() {
        for tree in r.iter_mut().rev() {
            // println!("{:?}", tree);
            tree.compare_height_i(&mut max_height_map, &mut visible_tree_cnt);
            tree.compare_height_j(&mut max_height_map, &mut visible_tree_cnt);
        }
    }

    println!("visible tree count: {:?}", visible_tree_cnt);
}

// [[Tree { position: (0, 0), height: 3, visibility: 4 }, Tree { position: (0, 1), height: 0, visibility: 4 }, Tree { position: (0, 2), height: 3, visibility: 4 }, Tree { position: (0, 3), height: 7, visibility: 4 }, Tree { position: (0, 4), height: 3, visibility: 4 }],
//  [Tree { position: (1, 0), height: 2, visibility: 4 }, Tree { position: (1, 1), height: 5, visibility: 4 }, Tree { position: (1, 2), height: 5, visibility: 4 }, Tree { position: (1, 3), height: 1, visibility: 4 }, Tree { position: (1, 4), height: 2, visibility: 4 }],
//  [Tree { position: (2, 0), height: 6, visibility: 4 }, Tree { position: (2, 1), height: 5, visibility: 4 }, Tree { position: (2, 2), height: 3, visibility: 4 }, Tree { position: (2, 3), height: 3, visibility: 4 }, Tree { position: (2, 4), height: 2, visibility: 4 }],
//  [Tree { position: (3, 0), height: 3, visibility: 4 }, Tree { position: (3, 1), height: 3, visibility: 4 }, Tree { position: (3, 2), height: 5, visibility: 4 }, Tree { position: (3, 3), height: 4, visibility: 4 }, Tree { position: (3, 4), height: 9, visibility: 4 }],
//  [Tree { position: (4, 0), height: 3, visibility: 4 }, Tree { position: (4, 1), height: 5, visibility: 4 }, Tree { position: (4, 2), height: 3, visibility: 4 }, Tree { position: (4, 3), height: 9, visibility: 4 }, Tree { position: (4, 4), height: 0, visibility: 4 }]]
