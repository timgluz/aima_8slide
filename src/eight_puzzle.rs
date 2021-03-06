use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::search::{Action, SearchProblem};

pub const PUZZLE_SIZE: usize = 9;

const MIN_STATE_INDEX: usize = 0;
const MAX_STATE_INDEX: usize = 8;

pub type PuzzleStateRow = [u8; 9];
pub const DEFAULT_GOAL: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 0];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileDirection {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl TileDirection {
    pub fn delta(&self) -> i8 {
        match self {
            TileDirection::Up => -3,
            TileDirection::Down => 3,
            TileDirection::Left => -1,
            TileDirection::Right => 1,
            TileDirection::None => 0,
        }
    }
}

impl Into<Action> for TileDirection {
    fn into(self) -> Action {
        Action(self as usize)
    }
}

impl From<&Action> for TileDirection {
    fn from(action: &Action) -> Self {
        match action.0 {
            1 => TileDirection::Up,
            2 => TileDirection::Down,
            3 => TileDirection::Left,
            4 => TileDirection::Right,
            _ => TileDirection::None,
        }
    }
}

struct Tile {
    index: usize,
}

impl Tile {
    pub fn new(index: usize) -> Self {
        Tile::is_valid_index(index);

        Tile { index }
    }
    pub fn possible_actions(&self) -> Vec<TileDirection> {
        let mut actions = Vec::with_capacity(4);
        if self.can_go_up() {
            actions.push(TileDirection::Up)
        }
        if self.can_go_down() {
            actions.push(TileDirection::Down)
        }
        if self.can_go_left() {
            actions.push(TileDirection::Left)
        }
        if self.can_go_right() {
            actions.push(TileDirection::Right)
        }

        actions.shrink_to_fit();
        actions
    }

    pub fn neighbor(&self, action: &TileDirection) -> Tile {
        self.is_allowed_action(action);

        let neighbor: i8 = (self.index as i8) + action.delta();
        Tile::new(neighbor as usize)
    }

    fn check_action(&self, action: &TileDirection) -> bool {
        match action {
            TileDirection::Up => self.can_go_up(),
            TileDirection::Down => self.can_go_down(),
            TileDirection::Left => self.can_go_left(),
            TileDirection::Right => self.can_go_right(),
            TileDirection::None => true,
        }
    }

    pub fn is_valid_index(index: usize) {
        assert!(index >= MIN_STATE_INDEX && index <= MAX_STATE_INDEX);
    }

    fn is_allowed_action(&self, action: &TileDirection) {
        assert!(self.check_action(action));
    }

    fn can_go_up(&self) -> bool {
        self.index > 2
    }

    fn can_go_down(&self) -> bool {
        self.index < 6
    }

    fn can_go_right(&self) -> bool {
        self.index % 3 != 2
    }

    fn can_go_left(&self) -> bool {
        self.index % 3 != 0
    }
}

#[derive(Debug, Clone)]
pub struct EightPuzzleState {
    value: PuzzleStateRow,
}

impl EightPuzzleState {
    pub fn new(value: PuzzleStateRow) -> Self {
        EightPuzzleState { value }
    }

    pub fn value(&self) -> &PuzzleStateRow {
        &self.value
    }
    pub fn possible_actions(&self) -> Vec<TileDirection> {
        let blank_squared_index = self.find_blank_square();

        Tile::new(blank_squared_index).possible_actions()
    }

    pub fn next_state(&self, action: &TileDirection) -> Self {
        let blank_squared_index = self.find_blank_square();
        let blank_square = Tile::new(blank_squared_index);
        assert!(blank_square.check_action(action));

        let neighbor = blank_square.neighbor(action);
        let mut new_value = self.value.clone();

        new_value.swap(blank_square.index, neighbor.index);

        EightPuzzleState::new(new_value)
    }

    ///  is_solveable() checks inversions.
    ///  Given a board, an inversion is any pair of tiles i and j where i < j
    ///  but i appears after j when considering the board in row-major order
    ///  (row 0, followed by row 1, and so forth).
    ///  source:
    ///  https://www.cs.princeton.edu/courses/archive/spring18/cos226/assignments/8puzzle/index.html
    pub fn is_solveable(&self) -> bool {
        let mut inversion = 0;

        let state = self.value();
        let state_size = state.len();

        for i in 0..state_size {
            for j in (i + 1)..state_size {
                if state[i] > state[j] && state[i] != 0 && state[j] != 0 {
                    inversion += 1
                }
            }
        }

        inversion % 2 == 0
    }

    fn find_blank_square(&self) -> usize {
        self.value().iter().position(|&x| x == 0).unwrap()
    }
}

pub struct EightPuzzle {
    state: EightPuzzleState,
    goal: EightPuzzleState,
}

impl EightPuzzle {
    pub fn new(initial_state: EightPuzzleState) -> Self {
        EightPuzzle {
            state: initial_state,
            goal: EightPuzzleState::new(DEFAULT_GOAL),
        }
    }

    pub fn from_row(state_row: PuzzleStateRow) -> Self {
        EightPuzzle {
            state: EightPuzzleState::new(state_row),
            goal: EightPuzzleState::new(DEFAULT_GOAL),
        }
    }

    pub fn state(&self) -> &EightPuzzleState {
        &self.state
    }

    // returns a problem from goal
    pub fn reverse_problem(&self) -> Self {
        EightPuzzle {
            state: EightPuzzleState::new(DEFAULT_GOAL),
            goal: self.state.clone(),
        }
    }

    // returns the heuristic value for a given state.
    // here it is the number of misplaces tiles
    fn h(&self) -> u8 {
        // TODO:
        self.state()
            .value()
            .iter()
            .zip(self.goal.value().iter())
            .map(|(s, g)| if s != g { 1 } else { 0 })
            .sum()
    }
}

impl SearchProblem for EightPuzzle {
    fn actions(&self) -> Vec<Action> {
        self.state()
            .possible_actions()
            .iter()
            .map(|&direction| direction.into())
            .collect()
    }

    fn result(&self, action: &Action) -> Box<dyn SearchProblem> {
        let direction = TileDirection::from(action);
        let solution = EightPuzzle::new(self.state.next_state(&direction));
        Box::new(solution)
    }

    fn test_goal(&self) -> bool {
        self.state.value() == self.goal.value()
    }

    // path cost of 8slide is depth plus one move from the depth
    fn path_cost(&self) -> u32 {
        1
    }

    fn value(&self) -> u32 {
        0
    }

    fn as_string(&self) -> String {
        format!("EightPuzzle(state: {:?}", self.state.value())
    }

    fn hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for EightPuzzle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().value().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blank_square_can_not_go_up_from_1st_row() {
        assert_eq!(false, Tile::new(0).can_go_up());
        assert_eq!(false, Tile::new(1).can_go_up());
        assert_eq!(false, Tile::new(2).can_go_up());
    }

    #[test]
    fn test_blank_square_can_go_up_from_2nd_row() {
        assert!(Tile::new(3).can_go_up());
        assert!(Tile::new(4).can_go_up());
        assert!(Tile::new(5).can_go_up());
    }

    #[test]
    fn test_blank_square_can_go_up_from_3rd_row() {
        assert!(Tile::new(6).can_go_up());
        assert!(Tile::new(7).can_go_up());
        assert!(Tile::new(8).can_go_up());
    }

    #[test]
    fn test_blank_square_can_not_go_down_from_3rd_row() {
        assert!(!Tile::new(6).can_go_down());
        assert!(!Tile::new(7).can_go_down());
        assert!(!Tile::new(8).can_go_down());
    }

    #[test]
    fn test_blank_square_can_go_down_from_2nd_row() {
        assert!(Tile::new(3).can_go_down());
        assert!(Tile::new(4).can_go_down());
        assert!(Tile::new(5).can_go_down());
    }

    #[test]
    fn test_blank_square_can_go_down_from_1st_row() {
        assert!(Tile::new(0).can_go_down());
        assert!(Tile::new(1).can_go_down());
        assert!(Tile::new(2).can_go_down());
    }

    #[test]
    fn test_blank_can_not_go_right_from_3rd_column() {
        assert!(!Tile::new(2).can_go_right());
        assert!(!Tile::new(5).can_go_right());
        assert!(!Tile::new(8).can_go_right());
    }

    #[test]
    fn test_blank_can_go_right_from_2nd_column() {
        assert!(Tile::new(1).can_go_right());
        assert!(Tile::new(4).can_go_right());
        assert!(Tile::new(7).can_go_right());
    }

    #[test]
    fn test_blank_can_go_right_from_1st_column() {
        assert!(Tile::new(0).can_go_right());
        assert!(Tile::new(3).can_go_right());
        assert!(Tile::new(6).can_go_right());
    }

    #[test]
    fn test_blank_can_not_go_left_from_1st_column() {
        assert!(!Tile::new(0).can_go_left());
        assert!(!Tile::new(3).can_go_left());
        assert!(!Tile::new(6).can_go_left());
    }

    #[test]
    fn test_blank_can_go_left_from_2nd_column() {
        assert!(Tile::new(1).can_go_left());
        assert!(Tile::new(4).can_go_left());
        assert!(Tile::new(7).can_go_left());
    }

    #[test]
    fn test_blank_can_go_left_from_3rd_column() {
        assert!(Tile::new(2).can_go_left());
        assert!(Tile::new(5).can_go_left());
        assert!(Tile::new(8).can_go_left());
    }
    #[test]
    fn test_possible_actions_from_0() {
        let blank = Tile::new(0);

        assert_eq!(
            vec![TileDirection::Down, TileDirection::Right],
            blank.possible_actions()
        );
    }

    #[test]
    fn test_possible_actions_from_1() {
        assert_eq!(
            vec![
                TileDirection::Down,
                TileDirection::Left,
                TileDirection::Right
            ],
            Tile::new(1).possible_actions()
        )
    }

    #[test]
    fn test_possible_actions_from_2() {
        assert_eq!(
            vec![TileDirection::Down, TileDirection::Left],
            Tile::new(2).possible_actions()
        )
    }

    #[test]
    fn test_possible_actions_from_6() {
        assert_eq!(
            vec![TileDirection::Up, TileDirection::Right],
            Tile::new(6).possible_actions()
        );
    }

    #[test]
    fn test_possible_actions_from_8() {
        assert_eq!(
            vec![TileDirection::Up, TileDirection::Left],
            Tile::new(8).possible_actions()
        );
    }

    #[test]
    fn test_init_new_puzzle() {
        let puzzle = EightPuzzle::from_row([1, 2, 3, 4, 5, 6, 7, 8, 0]);

        assert!(DEFAULT_GOAL == puzzle.state().value);
    }

    #[test]
    fn test_find_blank_square_at_the_beginning() {
        let puzzle = EightPuzzleState::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(0, puzzle.find_blank_square());
    }

    #[test]
    fn test_find_blank_square_at_the_middle() {
        let puzzle = EightPuzzleState::new([1, 2, 3, 4, 0, 5, 6, 7, 8]);

        assert_eq!(4, puzzle.find_blank_square());
    }

    #[test]
    fn test_find_blank_square_at_the_end() {
        let puzzle = EightPuzzleState::new([1, 2, 3, 4, 5, 6, 7, 8, 0]);

        assert_eq!(8, puzzle.find_blank_square());
    }

    #[test]
    fn test_eight_puzzle_actions_from_default_goal() {
        let puzzle = EightPuzzle::from_row(DEFAULT_GOAL);
        let up_action: Action = TileDirection::Up.into();
        let left_action: Action = TileDirection::Left.into();

        assert_eq!(vec![up_action, left_action], puzzle.actions())
    }

    #[test]
    fn test_eight_puzzle_result_with_valid_action() {
        let puzzle = EightPuzzle::from_row(DEFAULT_GOAL);
        let action: Action = TileDirection::Up.into();

        let res = puzzle.result(&action);
        assert_eq!(0, res.value());
    }

    #[test]
    #[should_panic]
    fn test_eight_puzzle_result_with_invalid_action() {
        let puzzle = EightPuzzle::from_row(DEFAULT_GOAL);
        let action: Action = TileDirection::Down.into();

        puzzle.result(&action);
    }

    #[test]
    fn test_eight_puzzle_goal_test_with_final_goal() {
        let puzzle = EightPuzzle::from_row(DEFAULT_GOAL);

        assert!(puzzle.test_goal());
    }

    #[test]
    fn test_eight_puzzle_goal_test_find_random_state() {
        let puzzle = EightPuzzle::from_row([1, 2, 0, 3, 4, 5, 6, 7, 8]);

        assert!(!puzzle.test_goal());
    }

    #[test]
    fn test_eight_puzzle_check_solvability_with_final_goal() {
        let puzzle = EightPuzzleState::new(DEFAULT_GOAL);

        assert!(puzzle.is_solveable());
    }

    #[test]
    fn test_eight_puzzle_check_solvability_with_random_state() {
        let puzzle = EightPuzzleState::new([1, 2, 3, 4, 0, 5, 6, 7, 8]);

        assert!(puzzle.is_solveable());
    }

    #[test]
    fn eight_puzzle_check_solvability_with_unsolvable_state() {
        let puzzle = EightPuzzleState::new([7, 0, 2, 8, 5, 3, 6, 4, 1]);

        assert!(!puzzle.is_solveable())
    }

    #[test]
    fn test_eight_puzzle_h_with_final_goal() {
        let puzzle = EightPuzzle::from_row(DEFAULT_GOAL);

        assert_eq!(0, puzzle.h());
    }

    #[test]
    fn test_eight_puzzle_h_with_2squares_swapped() {
        let puzzle = EightPuzzle::from_row([1, 2, 3, 4, 5, 6, 7, 0, 8]);

        assert_eq!(2, puzzle.h());
    }
}
