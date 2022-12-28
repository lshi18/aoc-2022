/*
 * https://adventofcode.com/2022/day/15
 *
 * Refactor is required
 */
use std::collections::{HashMap, HashSet};
use std::fs;

fn part_1(sensors_closest_beacons: &HashMap<(i64, i64), (i64, i64)>, y: i64) -> u64 {
    let mut res_range = HashSet::new();
    let mut beacons_on_y = HashSet::new();

    for (sensor, cloest_beacon) in sensors_closest_beacons.iter() {
        let closest_distance = hamilton_distance(sensor, cloest_beacon);
        let delta_x: i64 = closest_distance as i64 - sensor.1.abs_diff(y) as i64;

        /*println!(
            "{:?}, {:?}, {:?}, delta_x: {}",
            sensor, cloest_beacon, closest_distance, delta_x
        );*/

        if delta_x >= 0 {
            let x_l = sensor.0 - delta_x as i64;
            let x_r = sensor.0 + delta_x as i64;

            res_range.extend(x_l..=x_r);
        }

        if y == cloest_beacon.1 {
            beacons_on_y.insert(cloest_beacon.0);
        }
    }

    res_range.retain(|e| !beacons_on_y.contains(e));
    res_range.len() as u64
}

fn part_2(sensors_closest_beacons: &HashMap<(i64, i64), (i64, i64)>, max_coord: i64) -> i64 {
    for y in 0..=max_coord {
        let mut res_range = vec![];
        for (sensor, cloest_beacon) in sensors_closest_beacons.iter() {
            let closest_distance = hamilton_distance(sensor, cloest_beacon);
            let delta_x: i64 = closest_distance as i64 - sensor.1.abs_diff(y) as i64;

            /*println!(
                "{:?}, {:?}, {:?}, delta_x: {}",
                sensor, cloest_beacon, closest_distance, delta_x
            );*/

            if delta_x >= 0 {
                let x_l = sensor.0 - delta_x as i64;
                let x_r = sensor.0 + delta_x as i64;

                res_range.push(x_l..=x_r);
            }
        }

        res_range.sort_by(|a, b| a.start().cmp(b.start()));
        //println!("res: {:?}, {}", res_range, y);

        let mut curr_end = i64::MAX;
        for i in res_range.iter() {
            if curr_end < *i.start() - 1 && (0..=max_coord).contains(&(i.start() - 1)) {
                curr_end = i.start() - 1;
                //println!("calc: x: {}, y: {}", i.start() - 1, y);
                break;
            } else if curr_end == i64::MAX || curr_end < *i.end() {
                //println!("wu: {}, {}", curr_end, *i.end());
                curr_end = *i.end();
            }
        }

        if (0..=max_coord).contains(&(curr_end)) {
            return curr_end * 4000000 + y;
        }
    }
    -1
}

fn main() {
    let input = String::from_utf8(fs::read("input-day15.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let re = regex::Regex::new(
        r"Sensor at x=([-]?[\d]+), y=([-]?[\d]+): closest beacon is at x=([-]?[\d]+), y=([-]?[\d]+)",
    ).unwrap();

    let input = input
        .trim_end()
        .lines()
        .map(|l| {
            let cap = re.captures_iter(l).nth(0).unwrap();
            (
                (
                    cap[1].parse::<i64>().unwrap(),
                    cap[2].parse::<i64>().unwrap(),
                ),
                (
                    cap[3].parse::<i64>().unwrap(),
                    cap[4].parse::<i64>().unwrap(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    //println!("input: {:?}", input);
    //
    // Part 1:
    //let test = (10, 26);

    let test = (2000000, 4725496);
    let res = part_1(&input, test.0);
    assert!(res == test.1, "Part 1 real: {}", res);

    // Part 2:
    // let test = (20, 56000011);
    let test = (4000000, 12051287042458);
    let res = part_2(&input, test.0);
    assert!(res == test.1, "Part 1 real: {}", res);
}

fn hamilton_distance(&(x1, y1): &(i64, i64), &(x2, y2): &(i64, i64)) -> u64 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}
