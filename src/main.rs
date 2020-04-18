mod actions;
mod eight_puzzle;
mod frontiers;
mod search;

use frontiers::{Frontier, StackFrontier};
use search::{SearchNode, SearchProblem};

pub fn depth_first_tree_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = StackFrontier::new();
    let mut explored: Vec<SearchNode> = vec![];

    frontier.add(root_node);
    loop {
        match frontier.remove() {
            None => break,
            Some(current_node) => {
                println!("current state: ${:?}", current_node.item());

                if current_node.item().test_goal() {
                    return Some(current_node.clone());
                }

                let child_nodes = current_node.expand();
                explored.push(current_node);

                for child_node in child_nodes.into_iter() {
                    if !explored.contains(&child_node) {
                        frontier.add(child_node);
                    }
                }
            }
        }
    }

    None
}

fn main() {
    println!("Hello, world!");
    //let test_row = [7, 2, 4, 5, 0, 6, 8, 3, 1];
    let test_row = [1, 2, 3, 4, 5, 6, 7, 0, 8];
    let initial_state = eight_puzzle::EightPuzzleState::new(test_row);
    let puzzle = Box::new(eight_puzzle::EightPuzzle::new(initial_state));

    match depth_first_tree_search(puzzle) {
        None => println!("no solution for {:?}", test_row),
        Some(node) => println!("Got solution after {:?}", node.depth()),
    };
}
