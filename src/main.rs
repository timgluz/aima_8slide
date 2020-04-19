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
                println!(
                    "step.{:?} {:?} - {:?}",
                    current_node.depth(),
                    current_node.action(),
                    current_node.item()
                );

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

fn solve_eight_puzzle(test_row: [u8;9]) {
    let initial_state = eight_puzzle::EightPuzzleState::new(test_row);
    if !initial_state.is_solveable() {
        println!("Unsolvable problem: {:?}", initial_state.value());
        return;
    }

    let puzzle = Box::new(eight_puzzle::EightPuzzle::new(initial_state));

    match depth_first_tree_search(puzzle) {
        None => println!("no solution for {:?}", test_row),
        Some(node) => println!(
            "Found solution after {:?} steps: {:?}",
            node.depth(),
            node.solution()
        ),
    };
}

fn main() {
    //let test_row = [1, 2, 3, 4, 0, 5, 7, 8, 6]; // 2-steps
    let test_row = [1, 2, 3, 7, 4, 5, 0, 8, 6];
    solve_eight_puzzle(test_row);
}
