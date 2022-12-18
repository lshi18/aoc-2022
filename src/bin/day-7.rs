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

fn cal_dir_size(root: &Dir, total_size: &mut usize) -> usize {
    let mut dir_size = 0;
    for node in root.children.values() {
        match node {
            Node::Dir(dir) => dir_size += cal_dir_size(dir, total_size),
            Node::File(size) => dir_size += size,
        }
    }

    if dir_size <= 100000 {
        *total_size += dir_size;
    }

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

    let mut total_size: usize = 0;
    cal_dir_size(&root_after_parsing, &mut total_size);

    println!("part 1 : {:?}", total_size);
}
