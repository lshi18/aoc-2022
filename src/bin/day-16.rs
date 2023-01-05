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
    valves: Vec<NodeIndex>,
    time_left: u32,
    cache: &mut HashMap<(NodeIndex, Vec<NodeIndex>, u32), (Vec<(NodeIndex, u32)>, u32)>,
) -> (Vec<(NodeIndex, u32)>, u32) {
    // println!("valves: {:?}", valves);

    /* if time_left <= 0 {
        return (max_path, max_pressure);
    }*/

    /*if let Some((m_path, m_press)) = cache.get(&(curr_node, valves.clone(), time_left)) {
        //println!("cache hit!");
        return (m_path.clone(), *m_press);
    };*/

    let (_, flow_rate) = g.node_weight(curr_node).unwrap();

    let open_curr_time = if *flow_rate == 0 { 0 } else { 1 };

    let curr_pressure = flow_rate * (time_left - open_curr_time);

    let mut max_path = vec![(curr_node, time_left)];
    let mut max_pressure = curr_pressure;

    if valves.len() == 0 {
        return (vec![(curr_node, time_left)], curr_pressure);
    }

    // opening curr_node's valve
    for (i, v) in valves.iter().enumerate() {
        let mut closed_valves = valves.clone();
        closed_valves.remove(i);

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
            cache,
        );

        // println!("{:?} -> {:?}, {}, {:?}", curr_node, v, dist, one_path);
        let total = curr_pressure + pressure_released;
        if total > max_pressure {
            max_pressure = total;
            one_path.push((curr_node, time_left));
            max_path = one_path;
        }
    }
    // println!("{:?}, {:?}", max_path, max_pressure);
    /*cache.insert(
        (curr_node, valves, time_left),
        (max_path.clone(), max_pressure),
    );*/
    (max_path, max_pressure)
}

fn main() {
    let input = String::from_utf8(fs::read("input-day16.txt").unwrap()).unwrap();
    //let input = String::from_utf8(fs::read("sample.txt").unwrap()).unwrap();

    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=([\d]+); .*to valve[s]? (.*)").unwrap();

    let mut g = Graph::<(Rc<String>, u32), ()>::new();

    let mut valves = vec![];

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
                valves.push(node_idx);
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

    println!(
        "valves: {:?}\ngraph: {:?}\n", /*distance_matrix: {:?}"*/
        valves, g, /*distance_matrix*/
    );
    /*
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
        *names_idx.get(&Rc::new(String::from("AA"))).unwrap(),
        //*names_idx.get(&Rc::new(String::from("PD"))).unwrap(),
        &distance_matrix,
        // filtered,
        valves,
        30,
        &mut cache,
    );

    let mut path_n = path
        .iter()
        .map(|(p, t)| (g.node_weight(*p).unwrap(), t))
        .collect::<Vec<_>>();
    path_n.reverse();
    println!("Path: {:?}\nmax_pressure: {}", path_n, max_pressure);

    // assert!(max_pressure == 1651);
}
