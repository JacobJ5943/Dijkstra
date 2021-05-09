use std::{borrow::Borrow, cell::RefCell, rc::Rc};

struct Node {
    pub neighbors:Vec<(isize, Rc<RefCell<Node>>)>,
    pub label: String,
}

impl Node {
    fn new(label:String, graph:&Graph, neighbors:Vec<String>) -> Node{
        Node {
            label,
            neighbors: vec![],
        }
    }
}
use std::collections::HashMap;
pub struct Graph {
// Pretty sure I need Rc here because the node can be owned by this vecor and the neighbor of other nodes.
// This isn't an issue though because I will never modify a node or this vector
    nodes:HashMap<String, Rc<RefCell<Node>>>
}

impl Graph {
    fn new(nodes: HashMap<String, Rc<RefCell<Node>>>) -> Self { 
        Graph {
            nodes
        }
    }

    fn add_node(node:Node, neighbors:Vec<String>) {

    }
}
enum Visit {
    UNVISITED,
    VISITED
}

impl Node {
    fn add_neighbor(&mut self, weight: isize, neighbord:&Rc<RefCell<Node>>) {
        self.neighbors.push((weight, Rc::clone(&neighbord)));
    }
}

pub fn create_wikipedia_example() -> Graph{
    let mut graph = Graph::new(HashMap::new());

    let node1 = Rc::new(RefCell::new(Node::new(String::from("1"), &graph, vec![])));
    let node2 = Rc::new(RefCell::new(Node::new(String::from("2"), &graph, vec![])));
    let node3 = Rc::new(RefCell::new(Node::new(String::from("3"), &graph, vec![])));
    let node4 = Rc::new(RefCell::new(Node::new(String::from("3"), &graph, vec![])));
    let node5 = Rc::new(RefCell::new(Node::new(String::from("3"), &graph, vec![])));
    let node6 = Rc::new(RefCell::new(Node::new(String::from("3"), &graph, vec![])));

    node1.borrow_mut().add_neighbor(7, &node2);
    node1.borrow_mut().add_neighbor(14, &node6);
    node1.borrow_mut().add_neighbor(9, &node3);

    node2.borrow_mut().add_neighbor(7, &node1);
    node2.borrow_mut().add_neighbor(15, &node4);
    node2.borrow_mut().add_neighbor(10, &node3);

    node3.borrow_mut().add_neighbor(9, &node1);
    node3.borrow_mut().add_neighbor(11, &node4);
    node3.borrow_mut().add_neighbor(10, &node2);
    node3.borrow_mut().add_neighbor(2, &node6);

    node4.borrow_mut().add_neighbor(11, &node3);
    node4.borrow_mut().add_neighbor(15, &node2);
    node4.borrow_mut().add_neighbor(6, &node5);

    node5.borrow_mut().add_neighbor(6, &node4);
    node5.borrow_mut().add_neighbor(9, &node6);


    node6.borrow_mut().add_neighbor(14, &node1);
    node6.borrow_mut().add_neighbor(2, &node3);
    node6.borrow_mut().add_neighbor(9, &node5);

    graph
}

pub fn djikstras_algorithm() {
    // @TODO Fill this graph in 
    // @TODO have this actually take agruments for tests and use
    let mut wikigraph = create_wikipedia_example();

    // 1. Label every node in the graph as unvisieted

}