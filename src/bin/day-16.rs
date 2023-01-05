use itertools::Itertools;
use petgraph::algo::floyd_warshall::floyd_warshall;
use petgraph::graph::{Graph, NodeIndex};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn maximum_pressure(
    g: &Graph<(String, u32), ()>,
    curr_node: NodeIndex,
    distance_matrix: &HashMap<(NodeIndex, NodeIndex), u32>,
    //valves: Vec<NodeIndex>,
    valves: HashSet<NodeIndex>,
    time_left: u32,
) -> (Vec<(NodeIndex, u32)>, u32) {
    let (_, flow_rate) = g.node_weight(curr_node).unwrap();

    let open_curr_time = open_curr_time(*flow_rate);

    let curr_pressure = flow_rate * (time_left - open_curr_time);

    let mut max_path = vec![(curr_node, time_left)];
    let mut max_pressure = curr_pressure;

    // opening curr_node's valve
    for (_i, v) in valves.iter().enumerate() {
        let mut closed_valves = valves.clone();
        closed_valves.remove(v);

        let dist = *distance_matrix.get(&(curr_node, *v)).unwrap();

        if time_left <= dist + open_curr_time {
            continue;
        }

        let (mut one_path, pressure_released) = maximum_pressure(
            g,
            *v,
            distance_matrix,
            closed_valves,
            time_left - open_curr_time - dist,
        );

        // println!("{:?} -> {:?}, {}, {:?}", curr_node, v, dist, one_path);
        let total = curr_pressure + pressure_released;
        if total > max_pressure {
            max_pressure = total;
            one_path.push((curr_node, time_left));
            max_path = one_path;
        }
    }
    (max_path, max_pressure)
}

fn open_curr_time(flow_rate: u32) -> u32 {
    if flow_rate == 0 {
        0
    } else {
        1
    }
}

fn part_1(
    g: &Graph<(String, u32), ()>,
    distance_matrix: &HashMap<(NodeIndex, NodeIndex), u32>,
    names_idx: &HashMap<String, NodeIndex>,
    valves: HashSet<NodeIndex>,
) {
    /*
    println!(
        "valves: {:?}\ngraph: {:?}\n", /*distance_matrix: {:?}"*/
        valves, g, /*distance_matrix*/
    );
        let visited = vec!["BC", "OF", "OQ", "TN", "BV", "HR", "PD"]
            .iter()
            .map(|&s| *names_idx.get(&Rc::new(String::from(s))).unwrap())
            .collect::<Vec<_>>();
        let filtered = valves
            .into_iter()
            .filter(|i| !visited.contains(i))
            .collect::<Vec<_>>();
    */

    let (path, max_pressure) = maximum_pressure(
        &g,
        *names_idx.get(&String::from("AA")).unwrap(),
        //*names_idx.get(&Rc::new(String::from("PD"))).unwrap(),
        &distance_matrix,
        // filtered,
        valves.clone(),
        30,
    );

    let mut path_n = path
        .iter()
        .map(|(p, t)| (g.node_weight(*p).unwrap(), t))
        .collect::<Vec<_>>();
    path_n.reverse();
    println!("Path: {:?}\nmax_pressure: {}", path_n, max_pressure);

    // assert!(max_pressure == 1651, "sample answer");
    assert!(max_pressure == 1796, "part 1");
}

fn part_2(
    g: &Graph<(String, u32), ()>,
    distance_matrix: &HashMap<(NodeIndex, NodeIndex), u32>,
    names_idx: &HashMap<String, NodeIndex>,
    valves: HashSet<NodeIndex>,
) {
    let half_size = valves.len() / 2;

    let mut part_2_max_pressure = 0;
    let mut paths: (Vec<(NodeIndex, u32)>, Vec<(NodeIndex, u32)>) = (vec![], vec![]);
    for size in 1..=half_size {
        for sub_valves_1 in valves.clone().into_iter().combinations(size) {
            let sub_valves_1 = sub_valves_1.clone().into_iter().collect::<HashSet<_>>();
            let sub_valves_2 = valves
                .difference(&sub_valves_1)
                .map(|x| *x)
                .collect::<HashSet<_>>();

            let (max_path_1, max_pressure_1) = maximum_pressure(
                &g,
                *names_idx.get(&String::from("AA")).unwrap(),
                &distance_matrix,
                sub_valves_1,
                26,
            );

            let (max_path_2, max_pressure_2) = maximum_pressure(
                &g,
                *names_idx.get(&String::from("AA")).unwrap(),
                &distance_matrix,
                sub_valves_2,
                26,
            );

            let mp = max_pressure_1 + max_pressure_2;
            if mp > part_2_max_pressure {
                part_2_max_pressure = mp;
                paths = (max_path_1, max_path_2);
            }
        }
    }

    println!("Path: {:?}\nmax_pressure: {}", paths, part_2_max_pressure);
}

fn main() {
    let input = String::from_utf8(fs::read("input-day16.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=([\d]+); .*to valve[s]? (.*)").unwrap();

    let mut g = Graph::<(String, u32), ()>::new();
    // let mut valves = vec![];
    let mut valves = HashSet::new();
    let mut names_idx = HashMap::new();
    let mut edges = vec![];

    for line in input.trim().lines() {
        for cap in re.captures_iter(line) {
            // println!("{}, {}, {}", &cap[1], &cap[2], &cap[3]);
            let valve_name = String::from(&cap[1]);
            let flow_rate: u32 = cap[2].parse().unwrap();
            let neighbours = cap[3]
                .split(", ")
                .map(|s| String::from(s))
                .collect::<Vec<_>>();

            let node_idx = g.add_node((valve_name.clone(), flow_rate));

            neighbours.iter().for_each(|n| {
                edges.push((valve_name.clone(), n.clone()));
            });

            names_idx.insert(valve_name.clone(), node_idx);
            if flow_rate > 0 {
                valves.insert(node_idx);
            }
        }
    }

    g.extend_with_edges(
        edges
            .iter()
            .map(|(e1, e2)| (*names_idx.get(e1).unwrap(), *names_idx.get(e2).unwrap())),
    );

    let distance_matrix = floyd_warshall(&g, |_e| 1u32).unwrap();
    let t = (0..10).permutations(2).collect::<Vec<_>>().len();
    println!("l: {}", t);

    //part_1(&g, &distance_matrix, &names_idx, valves.clone());
    part_2(&g, &distance_matrix, &names_idx, valves.clone());
}
