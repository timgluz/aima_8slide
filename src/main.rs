mod eight_puzzle;
mod search;

use eight_puzzle::{PuzzleStateRow, TileDirection, PUZZLE_SIZE};
use search::uninformed::*;
use search::{SearchNode, SearchProblem};

enum SearchAlgorithm {
    DepthFirst,
    BreadthFirst,
    UniformCost,
    DepthLimited,
}

fn solve_eight_puzzle(test_row: [u8; 9], algorithm: SearchAlgorithm, max_depth: usize) {
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
        SearchAlgorithm::DepthLimited => depth_limited_search(puzzle, max_depth),
    };

    match maybe_solution {
        None => println!("no solution for {:?}", test_row),
        Some(node) => print_solution(&node),
    };
}

fn print_solution(node: &SearchNode) {
    println!("Found solution after {:?} steps. Path:", node.depth());

    for action in node.solution().iter() {
        let direction: TileDirection = action.into();
        match direction {
            TileDirection::None => print!("|-> "),
            _ => print!("{:?}, ", direction),
        }
    }
    println!("|");
}

const USAGE: &'static str = "
Usage:
    aima_8slide 1,2,3,4,5,0,7,8,6
    aima_8slide 1,2,3,7,4,5,0,8,6 depth_first

Algorithms available:
    depth_first - takes the last Action first, if tile can go to all 4 directions, then it would go right
    breadth_first - tries every action on the same level
    uniform_cost - takes cheapest (here shallowest) route first as route cost is constant
    depth_limited - recursively does depth_first until max depth has reached
";

const DEFAULT_ALGORITHM: SearchAlgorithm = SearchAlgorithm::BreadthFirst;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).expect(&USAGE);
    if command == "-h" || command == "--help" {
        print_usage();
    }

    let test_row = puzzle_from_string(command);
    let test_algo = algorithm_from_string(args.get(2).unwrap_or(&String::from("")));

    solve_eight_puzzle(test_row, test_algo, 10);
}

fn puzzle_from_string(row_str: &String) -> PuzzleStateRow {
    let row: Vec<u8> = row_str
        .split(|c| c == ',')
        .map(|c| c.trim().parse::<u8>().expect("Not number"))
        .collect();

    if row.len() != PUZZLE_SIZE {
        eprintln!("Error: puzzle must have {} items.", PUZZLE_SIZE);
        print_usage();
    };

    let mut puzzle_row: PuzzleStateRow = [0; PUZZLE_SIZE];
    for (i, &number) in row.iter().enumerate() {
        puzzle_row[i] = number;
    }

    puzzle_row
}

fn algorithm_from_string(algo_str: &String) -> SearchAlgorithm {
    match algo_str.trim().to_lowercase().as_str() {
        "depth_first" => SearchAlgorithm::DepthFirst,
        "breadth_first" => SearchAlgorithm::BreadthFirst,
        "uniform_cost" => SearchAlgorithm::UniformCost,
        "depth_limited" => SearchAlgorithm::DepthLimited,
        _ => DEFAULT_ALGORITHM,
    }
}

fn print_usage() {
    eprintln!("{}", USAGE);
    std::process::exit(1);
}
