mod actions;
mod eight_puzzle;
mod search;

use search::uniform::{depth_first_search, breadth_first_search};

fn solve_eight_puzzle(test_row: [u8;9]) {
    let initial_state = eight_puzzle::EightPuzzleState::new(test_row);
    if !initial_state.is_solveable() {
        println!("Unsolvable problem: {:?}", initial_state.value());
        return;
    }

    let puzzle = Box::new(eight_puzzle::EightPuzzle::new(initial_state));
    let solver = |puzzle| { depth_first_search(puzzle) };
    let solver2 = |puzzle| { breadth_first_search(puzzle) };

    match  solver2(puzzle) {
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
    solve_eight_puzzle(test_row);
}
