use std::collections::HashSet;
use std::fmt;
use std::fs;

struct Point(i32, i32, HashSet<(i32, i32)>);

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Point {
    fn make_move(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
            Direction::UpLeft => {
                self.0 -= 1;
                self.1 += 1
            }
            Direction::UpRight => {
                self.0 += 1;
                self.1 += 1
            }
            Direction::DownLeft => {
                self.0 -= 1;
                self.1 -= 1
            }
            Direction::DownRight => {
                self.0 += 1;
                self.1 -= 1
            }
        }

        self.2.insert((self.0, self.1));
    }

    fn new() -> Point {
        let mut history = HashSet::new();
        history.insert((0, 0));
        Point(0, 0, history)
    }

    fn generate_tail_move(&self, head: &Point) -> Option<Direction> {
        match (self.0 - head.0, self.1 - head.1) {
            (0, 0)
            | (1, 0)
            | (-1, 0)
            | (0, 1)
            | (0, -1)
            | (1, 1)
            | (1, -1)
            | (-1, 1)
            | (-1, -1) => None,

            (0, 2) => Some(Direction::Down),
            (0, -2) => Some(Direction::Up),
            (2, 0) => Some(Direction::Left),
            (-2, 0) => Some(Direction::Right),
            // A | E | J
            (1, 2) | (2, 1) | (2, 2) => Some(Direction::DownLeft),
            // B | F | K
            (-1, 2) | (-2, 1) | (-2, 2) => Some(Direction::DownRight),
            // C | G | L
            (1, -2) | (2, -1) | (2, -2) => Some(Direction::UpLeft),
            // D | I | M
            (-1, -2) | (-2, -1) | (-2, -2) => Some(Direction::UpRight),
            _ => unreachable!(),
        }
    }
}

// .KB.AJ..
// .F...E.
// ...H...
// .I...G.
// .MD.CL.

fn expand_moves(cmd_str: &str) -> Vec<Direction> {
    let direction_steps = cmd_str.split(' ').collect::<Vec<_>>();
    let direction = match *direction_steps.first().unwrap() {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => unreachable!(),
    };
    let steps = direction_steps.last().unwrap().parse::<usize>().unwrap();

    vec![direction; steps]
}

fn part_1() {
    let input = String::from_utf8(fs::read("input-day9.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let parsed = input.trim_end().split('\n').collect::<Vec<_>>();

    let head_moves = parsed
        .iter()
        .flat_map(|&s| expand_moves(s))
        .collect::<Vec<_>>();

    let mut t_head = Point::new();
    let mut t_tail = Point::new();

    let mut iter = head_moves.into_iter();
    while let Some(h_direction) = iter.next() {
        t_head.make_move(h_direction);
        if let Some(t_direction) = t_tail.generate_tail_move(&t_head) {
            t_tail.make_move(t_direction);
        }
    }

    let answer = t_tail.2.len();
    println!("Part 1 {:?}", answer);
    assert!(answer == 6376);
}

fn part_2() {
    let input = String::from_utf8(fs::read("input-day9.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let parsed = input.trim_end().split('\n').collect::<Vec<_>>();

    let head_moves = parsed
        .iter()
        .flat_map(|&s| expand_moves(s))
        .collect::<Vec<_>>();

    let mut head = Point::new();
    let mut tails = vec![];

    for _i in 1..=9 {
        tails.push(Point::new());
    }

    for (n_cmd, h_direction) in head_moves.into_iter().enumerate() {
        // println!("{}: {:?}", n_cmd + 1, h_direction);
        head.make_move(h_direction);
        // println!("H: {:?}", head);
        let mut prev = &head;
        let mut next_move: Option<Direction>;

        for (i, t) in tails.iter_mut().enumerate() {
            next_move = t.generate_tail_move(prev);

            if let Some(direction) = next_move {
                t.make_move(direction);
            }
            // println!("{}: after: {:?}", i + 1, t);
            prev = t;
        }
    }

    // println!("{:?}", tails);
    let answer = tails.pop().unwrap().2.len();

    println!("part 2: {}", answer);
    assert!(answer == 2607);
}

fn main() {
    part_1();
    part_2();
}
