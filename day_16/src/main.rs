mod tunnel_system;
use itertools::Itertools;
use tunnel_system::TunnelSystem;

fn parse_input<'a>() -> TunnelSystem<'a> {
    let lines = include_str!("input.txt").lines();
    let parsed = lines.map(|l| { 
        let (node_def, edge_def) = l.split_once(";").unwrap();
        let mut node_def_iter = node_def.split(" ");
        node_def_iter.next();

        let node_label = node_def_iter.next().unwrap();
        node_def_iter.next();
        node_def_iter.next();

        let flow_rate_str = node_def_iter.next().unwrap();
        let flow_rate_split = flow_rate_str.split_once("=").unwrap().1;

        let flow_rate: i32 = flow_rate_split.parse().unwrap();

        let mut adjacency_list_iter = edge_def.split(" ");
        adjacency_list_iter.next();
        adjacency_list_iter.next();
        adjacency_list_iter.next();
        adjacency_list_iter.next();
        adjacency_list_iter.next();

        let mut adjacency_list = vec![];
        while let Some(label) = adjacency_list_iter.next() {
            adjacency_list.push(label.replace(",", ""));
        }

        ((node_label, flow_rate), adjacency_list)
    }).collect::<Vec<_>>();

    TunnelSystem::new(parsed)
}

fn main() {
    let mut tunnel_system = parse_input();

    let (start_index, _) = tunnel_system.find_node_by_label("AA").unwrap();
    let part_1_releases = tunnel_system.find_maximum_release(start_index, 30);
    let part_1_release = part_1_releases.values().max().unwrap();

    println!("The maximum release value is: {part_1_release}");

    let max_releases = tunnel_system.find_maximum_release(start_index, 26);
    let part_2_release = max_releases
        .iter()
        .tuple_combinations()
        .filter(|(h, e)| h.0.is_disjoint(e.0))
        .map(|(h, e)| h.1 + e.1)
        .max()
        .unwrap();


    println!("The maximum release with an elephant is {}", part_2_release)
}


