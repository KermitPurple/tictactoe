const BOARD_SIZE: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    X,
    O,
    Empty,
}

struct TicTacToeBoard {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl TicTacToeBoard {
    fn new() -> Self {
        use Cell::*;
        Self { board: [[Empty; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    fn full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    fn winner(&self) -> Cell {
        // horizontal
        for y in 0..BOARD_SIZE {
            let player = self.board[y][0];
            if self.board[y].iter().all(|&cell| cell == player) {
                return player
            }
        }
        // vertical
        'outer: for x in 0..BOARD_SIZE {
            let player = self.board[0][x];
            for y in 1..BOARD_SIZE {
                if self.board[y][x] != player {
                    continue 'outer;
                }
            }
            return player;
        }
        // diagonal \
        let player = self.board[0][0];
        let mut won = true;
        for i in 1..BOARD_SIZE {
            if self.board[i][i] != player {
                won = false;
                break;
            }
        }
        if won {
            return player;
        }
        // diagonal /
        let player = self.board[0][ BOARD_SIZE - 1];
        won = true;
        for i in 1..BOARD_SIZE {
            if self.board[i][(BOARD_SIZE - 1) - i] != player {
                won = false;
                break;
            }
        }
        if won {
            player
        } else {
            Cell::Empty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full(){
        use Cell::*;
        let board = TicTacToeBoard {
            board: [
                [X; 3],
                [O, X, O],
                [X, Empty, O]
            ]
        };
        assert!(!board.full());
        let board = TicTacToeBoard {
            board: [[X; 3]; 3]
        };
        assert!(board.full());
        let board = TicTacToeBoard {
            board: [[Empty; 3]; 3]
        };
        assert!(!board.full());
    }

    #[test]
    fn test_winner() {
        use Cell::*;
        // horizontal
        let board = TicTacToeBoard {
            board: [
                [X, Empty, X],
                [X; 3],
                [Empty; 3]
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToeBoard {
            board: [
                [O; 3],
                [Empty; 3],
                [Empty; 3]
            ]
        };
        assert_eq!(board.winner(), O);
        // vertical
        let board = TicTacToeBoard {
            board: [
                [X, O, X],
                [X, Empty, X],
                [X, O, Empty],
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToeBoard {
            board: [
                [X, O, Empty],
                [Empty, O, X],
                [X, O, Empty],
            ]
        };
        assert_eq!(board.winner(), O);
        // diagonal \
        let board = TicTacToeBoard {
            board: [
                [X, O, O],
                [O, X, O],
                [O, O, X]
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToeBoard {
            board: [
                [O, X, X],
                [X, O, X],
                [X, X, O]
            ]
        };
        assert_eq!(board.winner(), O);
        // diagonal /
        let board = TicTacToeBoard {
            board: [
                [O, O, X],
                [O, X, O],
                [X, O, X]
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToeBoard {
            board: [
                [O, X, O],
                [X, O, X],
                [O, X, X]
            ]
        };
        assert_eq!(board.winner(), O);
        // no winner
        let board = TicTacToeBoard {
            board: [
                [X, O, X],
                [X, O, X],
                [O, X, O]
            ]
        };
        assert_eq!(board.winner(), Empty);
        let board = TicTacToeBoard {
            board: [[Empty; 3]; 3]
        };
        assert_eq!(board.winner(), Empty);
    }
    
}

fn main() {
}
