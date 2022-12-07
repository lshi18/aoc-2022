use std::cmp::Reverse;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = String::from_utf8(fs::read("input.txt")?).unwrap();
    let mut c = input
        .split("\n\n")
        .map(|l| l.trim_end().split("\n"))
        .map(|l| l.map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<_>>();
    c.sort_by_key(|a| Reverse(*a));
    println!(
        "max: {:?}
max_three: {:?}
max_three_total: {:?}",
        c[0],
        &c[0..3],
        &c[0..3].iter().sum::<u32>()
    );
    Ok(())
}
