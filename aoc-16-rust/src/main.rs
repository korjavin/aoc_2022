// Struct that holds graph data
// vec of nodes with int data and str name
// vec of adjacencies between nodes

// Struct that holds node data
// int data and str name
#[derive(Debug,Clone)]
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
        let node = Node {
            name,
            flow_rate,
            open: false,
        };
        self.nodes.push(node);
    }
    // add edge to graph
    fn add_edge(&mut self, node1: usize, node2: usize, weight: i32) {
        self.adjacencies[node1][node2] = weight;
    }

    fn print_graph(&self) {
        for i in 0..self.nodes.len() {
            println!("Node: {}", self.nodes[i].name);
            for j in 0..self.nodes.len() {
                if self.adjacencies[i][j] != 0 {
                    println!("Edge: {} to {} with weight {}", self.nodes[i].name, self.nodes[j].name, self.adjacencies[i][j]);
                }
            }
        }
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

    fn get_node_by_name(&self, name: &str) -> usize {
        for i in 0..self.nodes.len() {
            if self.nodes[i].name == name {
                return i;
            }
        }
        0
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
            accum_flow += self.calculate_flow();
            match m {
                Choice::Node(node) => {
                    curr_node = self.get_node_by_name(&node);
                }
                Choice::Valve => {
                    self.open_node(curr_node);
                }
            }
        }
        accum_flow
	}
}



enum Choice {
    Node(String),
    Valve,
}



#[test]
fn test_graph() {
    let mut graph = Graph::new_graph(5);
    graph.add_node( "AA".to_string(),1 );
    graph.add_node( "BB".to_string(),2 );
    graph.add_node( "CC".to_string(),1 );
    graph.add_node( "DD".to_string(),3 );
    graph.add_node( "EE".to_string(),1 );
    graph.add_edge(0, 1, 1);
    graph.add_edge(0, 2, 1);
    graph.add_edge(1, 3, 1);
    graph.add_edge(2, 3, 1);
    graph.add_edge(3, 4, 1);

    graph.open_node(2);
    assert_eq!(graph.calculate_flow(), 1);

    assert_eq!(graph.get_connected_nodes(0), vec![1, 2]);
    assert_eq!(graph.get_connected_nodes(1), vec![3]);
    assert_eq!(graph.get_connected_nodes(2), vec![3]);
    assert_eq!(graph.get_connected_nodes(3), vec![4]);
    assert_eq!(graph.get_connected_nodes(4), vec![]);

    assert_eq!(graph.get_node_by_name("EE"), 4);

    let moves = vec![
        Choice::Node("BB".to_string()), Choice::Valve,
        Choice::Node("DD".to_string()), Choice::Valve,
        Choice::Node("EE".to_string()), Choice::Valve,
    ];


    assert_eq!(graph.evaluate_result(moves), 8+6);






}


// add node to graph




fn main() {
    println!("Hello, world!");
}
