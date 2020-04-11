#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl Action {
    pub fn items() -> [Action; 4] {
        [Action::Up, Action::Down, Action::Left, Action::Right]
    }

    pub fn delta(&self) -> i8 {
        match self {
            Action::Up => -3,
            Action::Down => 3,
            Action::Left => -1,
            Action::Right => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
