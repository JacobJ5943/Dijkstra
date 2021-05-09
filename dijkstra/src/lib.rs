use std::{borrow::Borrow, cell::RefCell, collections::HashMap};
use std::cmp::Reverse;
use petgraph::graphmap::UnGraphMap;

pub fn create_pet_wiki_graph() -> UnGraphMap<isize, isize>{
    let mut deps:UnGraphMap<isize, isize> = UnGraphMap::new();
    let n1 = deps.add_node(1);
    let n2 = deps.add_node(2);
    let n3 = deps.add_node(3);
    let n4 = deps.add_node(4);
    let n5 = deps.add_node(5);
    let n6 = deps.add_node(6);
    deps.add_edge(1, 2, 7);
    deps.add_edge(1, 6, 14);
    deps.add_edge(1, 3, 9);
    deps.add_edge(2, 3, 10);
    deps.add_edge(2, 4, 15);
    deps.add_edge(3, 4, 11);
    deps.add_edge(3, 6, 2);
    deps.add_edge(4, 5, 6);
    deps.add_edge(5, 6, 9);
    /*
    deps.extend_with_edges(&[
        (n1,n2,7),(n1,n6,14),(n1,n3,9),
        (n2,n1,7),(n2,n3,10),(n2,n4,15),
        (n3,n1,0),(n3,n2,10),(n3,n4,11),(n3,n6,2),
        (n4,n5,6),(n4,n3,11),(n4,n2,15),
        (n5,n4,6),(n5,n6,9),
        (n6,n1,14),(n6,n3,2),(n6,n5,9),
    ]);
    */
    deps
}


#[derive(Debug, PartialEq, Eq)]
pub enum Visit {
    VISITED,
    UNVISITED
}
type ReverseTracking = Reverse<Tracking>;
#[derive(Debug, Eq)]
struct Tracking {
    label:isize,
    state:Visit,
    distance:isize,
}

impl Tracking {
    fn new(label:isize, state:Visit) -> Tracking {
        Tracking {
            label,
        state,
        distance:isize::MAX
        }
    }
}

impl Ord for Tracking {
    fn cmp(&self, other:&Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Tracking {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tracking {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

use std::cmp::Ordering;

use std::collections::BinaryHeap;
use std::rc::Rc;
pub fn djikstras_algorithm(starting_node:isize, target:isize) -> isize {
    //let target = "6";


    // @TODO Fill this graph in 
    // @TODO have this actually take agruments for tests and use
    // @TODO using the GraphMap will probably be a lot better than this
    let wikigraph = create_pet_wiki_graph();

    let mut tracking_map: HashMap<isize, Rc<RefCell<ReverseTracking>>> = HashMap::new();
    let mut unvisited: BinaryHeap<Rc<RefCell<ReverseTracking>>> = BinaryHeap::with_capacity(wikigraph.node_count());
    for node_index in  wikigraph.nodes() {
        let mut tracking_item = Rc::new(RefCell::new(Reverse(Tracking::new(node_index, Visit::UNVISITED))));

        if tracking_item.borrow_mut().0.label == starting_node {
            tracking_item.borrow_mut().0.distance = 0;
        }

        unvisited.push(Rc::clone(&tracking_item));
        tracking_map.insert(node_index, tracking_item);

        

    }
    let mut final_result = isize::MAX;
    while unvisited.len() > 0 {
        let current_node = unvisited.pop().unwrap();
        current_node.borrow_mut().0.state = Visit::VISITED;

        let mut neighbors = wikigraph.edges(current_node.borrow_mut().0.label);
        for (a,b,c) in neighbors {
            let mut b_distance = &mut tracking_map.get_mut(&b).unwrap().borrow_mut().0.distance;
            if *b_distance > current_node.borrow_mut().0.distance + c {
                *b_distance = current_node.borrow_mut().0.distance + c;
            } 
        }

        if current_node.borrow_mut().0.label == target {
            final_result = current_node.borrow_mut().0.distance;
        }
    }
   // Done with the setup 
    final_result


}

use std::fmt;


// This will be a loop that gets refarctored out
fn djikstra_loop() {

}