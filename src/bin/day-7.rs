/*
 * Problem: https://adventofcode.com/2022/day/7
*/
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Node {
    Dir(Dir),
    File(usize),
}

// Dir struct could be removed and simplifed as part of the Node::Dir constructor.
#[derive(Debug)]
struct Dir {
    pub name: String,
    pub children: HashMap<String, Node>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Dir {
            name: name.to_string(),
            children: HashMap::new(),
        }
    }
}

fn handle_cmds(mut dirs: Vec<Dir>, cmds: &[&str]) -> Dir {
    for &cmd in cmds {
        //println!("{:?}", cmd);
        match cmd {
            "cd /" => {
                let mut len = dirs.len();
                while len > 1 {
                    len = len - 1;
                    let latest = dirs.pop().unwrap();
                    dirs.last_mut()
                        .unwrap()
                        .children
                        .insert(latest.name.clone(), Node::Dir(latest));
                }
            }
            "ls" => (),
            "cd .." => {
                // println!("before exit {:?}", dirs);
                let latest = dirs.pop().unwrap();
                dirs.last_mut()
                    .unwrap()
                    .children
                    .insert(latest.name.clone(), Node::Dir(latest));
                // println!("after exit {:?}", dirs);
            }
            var_cmd => match &var_cmd[0..3] {
                "cd " => {
                    let name = var_cmd[3..].to_string();

                    if let Node::Dir(child_dir) =
                        dirs.last_mut().unwrap().children.remove(&name).unwrap()
                    {
                        dirs.push(child_dir);
                    }
                    // println!("cd into {}", name);
                }
                "dir" => {
                    let name = &var_cmd[4..];
                    let new_dir = Dir::new(&name.to_string());
                    // Handle possible repeatedly executed "ls" under the
                    // same folder.
                    dirs.last_mut()
                        .unwrap()
                        .children
                        .insert(name.to_string(), Node::Dir(new_dir));
                }
                _file_str => {
                    // println!("file {:?}", var_cmd);
                    if let [size, name] = var_cmd.split(' ').collect::<Vec<_>>()[..] {
                        dirs.last_mut()
                            .unwrap()
                            .children
                            .insert(name.to_string(), Node::File(size.parse::<usize>().unwrap()));
                    }
                }
            },
        }
    }
    dirs.pop().unwrap()
}

trait SizeCalulator {
    fn calc(&mut self, dir_size: &usize);
}

struct DirTotalSizeCalculation {
    total_size: usize,
    limit: usize,
}

struct DirMinRequirentCalculation {
    curr_size: usize,
    target: usize,
}

impl SizeCalulator for DirTotalSizeCalculation {
    fn calc(&mut self, dir_size: &usize) {
        if *dir_size <= self.limit {
            self.total_size += *dir_size;
        }
    }
}

impl SizeCalulator for DirMinRequirentCalculation {
    fn calc(&mut self, dir_size: &usize) {
        if *dir_size < self.curr_size && *dir_size >= self.target {
            self.curr_size = *dir_size;
        }
    }
}
fn cal_dir_size<C: SizeCalulator>(root: &Dir, calculator: &mut C) -> usize {
    let mut dir_size = 0;
    for node in root.children.values() {
        match node {
            Node::Dir(dir) => dir_size += cal_dir_size(dir, calculator),
            Node::File(size) => dir_size += size,
        }
    }

    calculator.calc(&dir_size);
    dir_size
}

fn main() {
    let input = String::from_utf8(fs::read("input-day7.txt").unwrap()).unwrap();
    // let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();
    let mut parsed = input
        .split("$ ")
        .flat_map(|s| s.trim_end().split('\n'))
        .filter(|&s| !s.is_empty())
        .collect::<Vec<_>>();

    parsed.push("cd /");

    let root: Dir = Dir::new("/");
    let mut dir_stack = vec![];

    dir_stack.push(root);

    let root_after_parsing = handle_cmds(dir_stack, &parsed);

    // Part 1
    let mut part1_calculator = DirTotalSizeCalculation {
        limit: 100000,
        total_size: 0,
    };

    let root_dir_size = cal_dir_size(&root_after_parsing, &mut part1_calculator);

    println!("part 1 result : {:?}", part1_calculator.total_size);
    assert!(1845346 == part1_calculator.total_size);

    // Part 2
    let to_be_freed = 30000000 - (70000000 - root_dir_size);
    println!(
        "root_dir_size : {:?}, to_be_freed: {:?}",
        root_dir_size, to_be_freed
    );

    let mut part2_calculator = DirMinRequirentCalculation {
        target: to_be_freed,
        curr_size: usize::MAX,
    };

    cal_dir_size(&root_after_parsing, &mut part2_calculator);

    println!("part 2 result: {:?}", part2_calculator.curr_size);
    assert!(3636703 == part2_calculator.curr_size);
}
