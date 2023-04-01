use std::collections::HashSet;

struct Game {
    board: Vec<usize>,
    starting_index: usize,
}

impl Game {
    /// ```
    /// let board = vec![1, 2, 3, 0, 3, 2];
    /// let starting_index = 0;
    /// jump_game::Game::new(board, starting_index);
    /// ```
    pub fn new(board: Vec<usize>, starting_index: usize) -> Self {
        if board.len() == 0 {
            panic!("Board must have at least one element");
        }
        if starting_index >= board.len() {
            panic!("Starting index must be within bounds of the board");
        }
        if !board.iter().any(|&x| x == 0) {
            panic!("Board must contain at least one 0");
        }
        Self {
            board,
            starting_index,
        }
    }

    pub fn check_if_winnable(&self) -> bool {
        let mut stack = Vec::<isize>::new();
        let mut visited = HashSet::<isize>::new();

        stack.push(self.starting_index as isize);

        while let Some(current_index) = stack.pop() {
            if visited.contains(&current_index) {
                // we've been here already - prevent infinite loops
                continue;
            }
            if current_index < 0 {
                // out of bounds left
                visited.insert(current_index);
                continue;
            }

            match self.board.get(current_index as usize) {
                Some(0) => {
                    // WINNER!
                    return true;
                }
                Some(value) => {
                    // not a 0, but still in bounds
                    stack.push(current_index - (*value as isize));
                    stack.push(current_index + (*value as isize));
                }
                None => {
                    // out of bounds right
                }
            }

            visited.insert(current_index);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    #[should_panic]
    fn game_board_cannot_be_empty() {
        std::panic::set_hook(Box::new(|_| {}));
        Game::new(vec![], 0);
    }

    #[test]
    #[should_panic]
    fn game_board_must_have_a_0_present() {
        std::panic::set_hook(Box::new(|_| {}));
        Game::new(vec![1], 0);
    }

    #[test]
    #[should_panic]
    fn starting_position_must_not_be_out_of_bounds() {
        std::panic::set_hook(Box::new(|_| {}));
        Game::new(vec![1], 2);
    }

    #[test]
    fn handles_a_cyclical_board_without_panicking() {
        let game = Game::new(vec![1, 1, 1, 1, 0], 0);
        assert!(game.check_if_winnable());
    }

    #[test_case(vec![1, 2, 3, 0, 3, 2], 0, true)]
    #[test_case(vec![1, 2, 3, 0, 3, 2], 1, true)]
    #[test_case(vec![1, 2, 3, 0, 3, 2], 2, true)]
    #[test_case(vec![1, 2, 3, 0, 3, 2], 3, true)]
    #[test_case(vec![1, 2, 3, 0, 3, 2], 4, true)]
    #[test_case(vec![1, 2, 3, 0, 3, 2], 5, true)]
    #[test_case(vec![1, 7, 3, 0, 3, 2], 0, false)]
    #[test_case(vec![1, 7, 3, 0, 3, 2], 1, false)]
    #[test_case(vec![1, 7, 3, 0, 3, 2], 2, true)]
    #[test_case(vec![1, 7, 3, 0, 3, 2], 3, true)]
    #[test_case(vec![1, 7, 3, 0, 3, 2], 4, false)]
    #[test_case(vec![1, 7, 3, 0, 3, 2], 5, true)]
    #[test_case(vec![1, 1, 6, 0, 2, 2, 2], 0, false)]
    #[test_case(vec![1, 1, 6, 0, 2, 2, 2], 1, false)]
    #[test_case(vec![1, 1, 6, 0, 2, 2, 2], 2, false)]
    #[test_case(vec![1, 1, 6, 0, 2, 2, 2], 3, true)]
    #[test_case(vec![1, 1, 6, 0, 2, 2, 2], 4, false)]
    #[test_case(vec![1, 1, 6, 0, 2, 2, 2], 5, true)]
    #[test_case(vec![1, 1, 6, 0, 2, 2, 2], 6, false)]
    fn test_cases(board: Vec<usize>, starting_index: usize, expected: bool) {
        let game = Game::new(board, starting_index);
        assert_eq!(game.check_if_winnable(), expected);
    }
}
