use std::io::{self, BufRead};
// Struct that holds graph data
// vec of nodes with int data and str name
// vec of adjacencies between nodes

// Struct that holds node data
// int data and str name
#[derive(Debug, Clone)]
struct Node {
    flow_rate: i32,
    name: String,
    open: bool,
}
struct Graph {
    nodes: Vec<Node>,
    adjacencies: Vec<Vec<i32>>,
}

impl Graph {
    fn new_graph(size: usize) -> Graph {
        let nodes = vec![];
        let mut adjacencies = vec![];
        for i in 0..size {
            let mut adj = Vec::with_capacity(size);
            for j in 0..size {
                let z = 0;
                adj.push(z);
            }
            adjacencies.push(adj);
        }
        Graph { nodes, adjacencies }
    }
    // add node to graph
    fn add_node(&mut self, name: String, flow_rate: i32) {
        let nd = self.get_node_by_name(&name);
        let node = Node {
            name,
            flow_rate,
            open: false,
        };
        match nd {
            Some(n) => {
                self.nodes[n].flow_rate = flow_rate;
            }
            None => {
                self.nodes.push(node);
            }
        }
    }
    // add edge to graph
    fn add_edge(&mut self, node1: usize, node2: usize, weight: i32) {
        self.adjacencies[node1][node2] = weight;
    }

    fn print_graph(&self) {
        for i in 0..self.nodes.len() {
            println!("Node: {} {}", self.nodes[i].name, self.nodes[i].flow_rate);
            for j in 0..self.nodes.len() {
                if self.adjacencies[i][j] != 0 {
                    println!("Edge: {} to {}", self.nodes[i].name, self.nodes[j].name);
                }
            }
        }
        println!("----------------");
    }

    fn get_connected_nodes(&self, node: usize) -> Vec<usize> {
        let mut connected_nodes = vec![];
        for i in 0..self.nodes.len() {
            if self.adjacencies[node][i] != 0 {
                connected_nodes.push(i);
            }
        }
        connected_nodes
    }

    fn get_node_by_name(&self, name: &str) -> Option<usize> {
        for i in 0..self.nodes.len() {
            if self.nodes[i].name == name {
                return Some(i);
            }
        }
        None
    }

    fn open_node(&mut self, node: usize) {
        self.nodes[node].open = true;
    }

    // fn calculate flow of all the open nodes
    fn calculate_flow(&mut self) -> i32 {
        let mut sum = 0;
        for i in 0..self.nodes.len() {
            if self.nodes[i].open {
                sum += self.nodes[i].flow_rate;
            }
        }
        sum
    }

    fn evaluate_result(&mut self, moves: Vec<Choice>) -> i32 {
        let mut curr_node = 0;
        let mut accum_flow = 0;
        for m in moves {
            let rate = self.calculate_flow();
            accum_flow += rate;
            match m {
                Choice::Node(node) => match self.get_node_by_name(&node) {
                    Some(n) => {
                        curr_node = n;
                        self.open_node(n);
                    }
                    None => {
                        panic!("Node not found");
                    }
                },
                Choice::Valve => {
                    self.open_node(curr_node);
                }
            }
        }
        accum_flow
    }
    fn score(&self, node: usize, moves_left: i32) -> i32 {
        let node = &self.nodes[node];
        if node.open {
            return 0;
        }
        return node.flow_rate * moves_left;
    }
}

#[derive(Debug, Clone)]
enum Choice {
    Node(String),
    Valve,
}

#[test]
fn test_graph() {
    let mut graph = Graph::new_graph(5);
    graph.add_node("AA".to_string(), 0);
    graph.add_node("AA".to_string(), 1);
    graph.add_node("AA".to_string(), 1);
    graph.add_node("AA".to_string(), 1);
    graph.add_node("AA".to_string(), 1);
    graph.add_node("BB".to_string(), 0);
    graph.add_node("BB".to_string(), 2);
    graph.add_node("CC".to_string(), 1);
    graph.add_node("DD".to_string(), 3);
    graph.add_node("EE".to_string(), 1);
    graph.add_edge(0, 1, 1);
    graph.add_edge(0, 2, 1);
    graph.add_edge(1, 3, 1);
    graph.add_edge(2, 3, 1);
    graph.add_edge(3, 4, 1);

    assert_eq!(graph.get_connected_nodes(0), vec![1, 2]);
    assert_eq!(graph.get_connected_nodes(1), vec![3]);
    assert_eq!(graph.get_connected_nodes(2), vec![3]);
    assert_eq!(graph.get_connected_nodes(3), vec![4]);
    assert_eq!(graph.get_connected_nodes(4), vec![]);

    assert_eq!(graph.get_node_by_name("EE"), Some(4));

    let moves = vec![
        Choice::Node("BB".to_string()),
        Choice::Valve,
        Choice::Node("DD".to_string()),
        Choice::Valve,
        Choice::Node("EE".to_string()),
    ];

    assert_eq!(graph.evaluate_result(moves), 8 + 6);
}

fn main() {
    let stdin = io::stdin();

    let mut graph = Graph::new_graph(10); //size?

    for line in stdin.lock().lines() {
        // Parse lint into node, rate and edges
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let line = line.unwrap();
        let words = line.split(|c| ";=, ".contains(c)).collect::<Vec<&str>>();
        let name = words[1].to_string();

        let mut me = graph.get_node_by_name(&name);
        let mut menumber;

        match me {
            Some(n) => {
                menumber = n;
                graph.add_node(name.clone(), words[5].parse::<i32>().unwrap());
            }
            None => {
                graph.add_node(name.clone(), words[5].parse::<i32>().unwrap());
                menumber = graph.get_node_by_name(&name).unwrap();
            }
        }

        for i in 11..words.len() {
            let nodename = words[i].to_string();
            if nodename == "" {
                continue;
            }
            let mut node = graph.get_node_by_name(&nodename);
            let mut nodenumber;
            match node {
                Some(n) => {
                    nodenumber = n;
                }
                None => {
                    graph.add_node(nodename.clone(), 0);
                    nodenumber = graph.get_node_by_name(&nodename).unwrap();
                }
            }
            graph.add_edge(menumber, nodenumber, 1);
        }
    }
    graph.print_graph();

    const MAX_MOVES: usize = 30;
    let mut current_node = graph.get_node_by_name("AA").unwrap();
    let mut game = Vec::<Choice>::new();

    for i in 1..MAX_MOVES {
        dbg!(i);
        let mut best_score = graph.score(current_node, (MAX_MOVES - i) as i32); //open this one
        dbg!(best_score);
        let mut best_move = Choice::Valve;
        let mut name = graph.nodes[current_node].name.clone();
        for node in graph.get_connected_nodes(current_node) {
            let mut score = graph.score(node, (MAX_MOVES - i) as i32);
            let name_conn = graph.nodes[node].name.clone();
            dbg!(&name);
            dbg!(score);
            if score > best_score {
                best_score = score;
                best_move = Choice::Node(name_conn.clone());
                name = name_conn.clone();
            }
        }
        game.push(best_move.clone());
        dbg!(&best_move);
        match best_move {
            Choice::Node(node) => {
                current_node = graph.get_node_by_name(&name).unwrap();
            }
            Choice::Valve => {
                graph.open_node(current_node);
            }
        }
    }
}
