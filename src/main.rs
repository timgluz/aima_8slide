mod actions;
mod eight_puzzle;
mod search;

use search::{SearchProblem, SearchNode};
use std::thread::current;

pub fn depth_first_tree_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode>{
    let initial_node = SearchNode::root(problem);
    let mut frontier: Vec<SearchNode> = vec![initial_node.clone()];
    let mut explored: Vec<SearchNode> = vec![];

    loop {
        match frontier.pop() {
            None => break,
            Some(current_node) => {
                println!("current state: ${:?}", current_node.item());

                if current_node.item().test_goal() { return Some(current_node.clone()); }

                let child_nodes = current_node.expand();
                explored.push(current_node);

                for child_node in child_nodes.into_iter() {
                    if !explored.contains(&child_node) {
                        frontier.push(child_node);
                    }
                }


            }
        }
    }

    None
}

fn main() {

    println!("Hello, world!");
    let test_row= [7, 2, 4, 5, 0, 6, 8, 3, 1];
    let initial_state = eight_puzzle::EightPuzzleState::new(test_row);
    let puzzle = Box::new(eight_puzzle::EightPuzzle::new(initial_state));

    match depth_first_tree_search(puzzle) {
        None => println!("no solution for {:?}", test_row),
        Some(node) => println!("Got solution after {:?}", node.depth())
    };
}
