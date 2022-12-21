/*
 * https://adventofcode.com/2022/day/10
 */
use std::fs;
#[derive(Debug)]
enum Cmd {
    Noop,
    Addx(i32),
}

fn parse_cmd(s: &str) -> Vec<Cmd> {
    s.trim_end()
        .split('\n')
        .map(|s| match s {
            "noop" => Cmd::Noop,
            _ => {
                if let ["addx", x] = s.split(' ').collect::<Vec<_>>()[..] {
                    Cmd::Addx(x.parse().unwrap())
                } else {
                    unreachable!()
                }
            }
        })
        .collect::<Vec<_>>()
}

impl Cmd {
    fn exec(
        &self,
        n_cycle: &mut u32,
        reg_x: &mut i32,
        signal_strength: &mut i32,
        crt_lines: &mut [u128],
    ) {
        match &self {
            Cmd::Noop => *n_cycle += 1,
            Cmd::Addx(_) => *n_cycle += 1,
        }

        calc_signal_strength(n_cycle, reg_x, signal_strength);
        calc_crt_lines(n_cycle, reg_x, crt_lines);
        // println!(
        //     "n_cycle {}, reg_x {} signal_strength {}",
        //     n_cycle, reg_x, signal_strength,
        // );

        if let Cmd::Addx(x) = &self {
            *n_cycle += 1;
            calc_signal_strength(n_cycle, reg_x, signal_strength);
            calc_crt_lines(n_cycle, reg_x, crt_lines);
            *reg_x += x;
            // println!(
            //     " n_cycle: {}, reg_x: {} signal_strength: {}",
            //     n_cycle, reg_x, signal_strength,
            // );
        };
    }
}

fn calc_signal_strength(n_cycle: &u32, reg_x: &i32, signal_strength: &mut i32) {
    let ss = match n_cycle {
        20 | 60 | 100 | 140 | 180 | 220 => (*n_cycle as i32) * reg_x,
        _ => 0,
    };

    *signal_strength += ss;
}
const PIXEL: u128 = 0b1000000000000000000000000000000000000000;
const SPRITE: u128 = 0b1110000000000000000000000000000000000000000;
const MASK: u128 = 0b0001111111111111111111111111111111111111111;

fn calc_crt_lines(n_cycle: &u32, reg_x: &i32, crt_lines: &mut [u128]) {
    // println!("cycle: {}, reg_x: {}", n_cycle, reg_x);
    let pos = (n_cycle - 1) % 40;
    let idx = (*n_cycle as usize - 1) / 40;

    let sprite_shift = if *reg_x >= -1 { *reg_x + 2 } else { 0 };

    let pixel = PIXEL >> pos;
    let sprite = SPRITE >> sprite_shift;

    // print!("pixel: ");
    // crt_print_line(pixel);
    // print!("sprite: ");
    // crt_print_line(sprite);

    // print!("thisline: ");
    // crt_print_line(pixel & sprite & MASK);
    let line = crt_lines[idx] | (pixel & sprite & MASK);
    crt_lines[idx] = line;
    if pos == 39 {
        crt_print_line(line)
    };
}

fn crt_print_line(input: u128) {
    let s = format!("{0:0>40b}", input);
    let s = s.replace("0", ".");
    let s = s.replace("1", "#");
    println!("{}", s);
}

fn main() {
    let input = String::from_utf8(fs::read("input-day10.txt").unwrap()).unwrap();
    // let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let cmds = parse_cmd(&input);

    let mut n_cycle: u32 = 0;
    let mut reg_x = 1;
    let mut signal_strength = 0;
    let mut crt_lines = vec![PIXEL; 6];

    for cmd in cmds.iter() {
        cmd.exec(
            &mut n_cycle,
            &mut reg_x,
            &mut signal_strength,
            &mut crt_lines,
        );
    }
    println!("");
    println!("Part 1: {:?}", signal_strength);
}
