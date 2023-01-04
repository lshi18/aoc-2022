use petgraph::algo::floyd_warshall::floyd_warshall;
use petgraph::graph::{Graph, NodeIndex};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::rc::Rc;

fn maximum_pressure(
    g: &Graph<(Rc<String>, u32), ()>,
    curr_node: NodeIndex,
    distance_matrix: &HashMap<(NodeIndex, NodeIndex), u32>,
    mut valves: HashSet<NodeIndex>,
    time_left: u32,
    mut cache: &HashMap<(NodeIndex, HashSet<NodeIndex>), (u32, u32)>,
) -> (Vec<(NodeIndex, u32)>, u32) {
    // println!("valves: {:?}", valves);

    let mut max_path = vec![];
    let mut max_pressure = 0;

    if time_left <= 0 {
        return (max_path, max_pressure);
    }

    let (_, flow_rate) = g.node_weight(curr_node).unwrap();
    valves.remove(&curr_node);
    let curr_pressure = flow_rate * time_left;

    if valves.len() == 0 {
        return (vec![(curr_node, time_left)], curr_pressure);
    }

    // opening curr_node's valve
    for v in valves.iter() {
        let dist = distance_matrix.get(&(curr_node, *v)).unwrap();

        let (mut one_path, pressure_released) = maximum_pressure(
            g,
            *v,
            distance_matrix,
            valves.clone(),
            time_left - 1 - dist,
            &mut cache,
        );

        // let mut hs = HashSet::new();
        // hs.insert(NodeIndex::new(7));
        // hs.insert(NodeIndex::new(4));
        // hs.insert(NodeIndex::new(9));
        // hs.insert(NodeIndex::new(1));
        // hs.insert(NodeIndex::new(2));

        // if valves == hs && curr_node == NodeIndex::new(3) {
        //     println!(
        //         "{:?}->{:?}: {}, {:?}, ",
        //         curr_node, v, dist, pressure_released,
        //     );
        // }

        let total = curr_pressure + pressure_released;
        if total > max_pressure {
            max_pressure = total;
            one_path.push((curr_node, time_left));
            max_path = one_path;
        }
    }
    // println!("{:?}, {:?}", max_path, max_pressure);
    (max_path, max_pressure)
}

fn main() {
    //let input = String::from_utf8(fs::read("input-day16.txt").unwrap()).unwrap();
    let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=([\d]+); .*to valve[s]? (.*)").unwrap();

    let mut g = Graph::<(Rc<String>, u32), ()>::new();

    let mut valves = HashSet::new();

    let mut names_idx = HashMap::new();

    let mut edges = vec![];

    for line in input.trim().lines() {
        for cap in re.captures_iter(line) {
            // println!("{}, {}, {}", &cap[1], &cap[2], &cap[3]);
            let valve_name = Rc::new(String::from(&cap[1]));
            let flow_rate: u32 = cap[2].parse().unwrap();
            let neighbours = cap[3]
                .split(", ")
                .map(|s| Rc::new(String::from(s)))
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

    let mut cache = HashMap::new();

    /*println!(
        "valves: {:?}\ngraph: {:?}\ndistance_matrix: {:?}",
        valves, g, distance_matrix
    );*/
    let (path, max_pressure) = maximum_pressure(
        &g,
        *names_idx.get(&Rc::new(String::from("AA"))).unwrap(),
        &distance_matrix,
        valves,
        30,
        &mut cache,
    );

    let mut path_n = path
        .iter()
        .map(|(p, time)| (g.node_weight(*p), time))
        .collect::<Vec<_>>();
    path_n.reverse();
    println!("Path: {:?}\nmax_pressure: {}", path_n, max_pressure);

    assert!(max_pressure == 1651);
}
