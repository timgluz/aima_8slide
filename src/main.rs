mod actions;
mod eight_puzzle;
mod search;

use search::uniform::*;

enum SearchAlgorithm {
    DepthFirst,
    BreadthFirst,
    UniformCost,
}

fn solve_eight_puzzle(test_row: [u8; 9], algorithm: SearchAlgorithm) {
    let initial_state = eight_puzzle::EightPuzzleState::new(test_row);
    if !initial_state.is_solveable() {
        println!("Unsolvable problem: {:?}", initial_state.value());
        return;
    }

    let puzzle = Box::new(eight_puzzle::EightPuzzle::new(initial_state));
    let maybe_solution = match algorithm {
        SearchAlgorithm::DepthFirst => depth_first_search(puzzle),
        SearchAlgorithm::BreadthFirst => breadth_first_search(puzzle),
        SearchAlgorithm::UniformCost => uniform_cost_search(puzzle),
    };

    match maybe_solution {
        None => println!("no solution for {:?}", test_row),
        Some(node) => println!(
            "Found solution after {:?} steps: {:?}",
            node.depth(),
            node.solution()
        ),
    };
}

// TODO: read Puzzle from CLI
// TODO: read algo from CLI
fn main() {
    let test_row = [1, 2, 3, 4, 0, 5, 7, 8, 6]; // 2-steps
                                                //let test_row = [1, 2, 3, 7, 4, 5, 0, 8, 6];
    solve_eight_puzzle(test_row, SearchAlgorithm::UniformCost);
}
