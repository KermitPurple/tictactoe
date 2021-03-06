const BOARD_SIZE: usize = 3;

use std::io::prelude::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    X,
    O,
    Empty,
}

impl Cell{
    fn swap(&mut self) {
        use Cell::*;
        *self = match self {
            X => O,
            O => X,
            Empty => Empty,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cell::X => "X",
            Cell::O => "O",
            Cell::Empty => " ",
        };
        write!(f, "{}", s)
    }
}

struct TicTacToe {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl TicTacToe {
    fn new() -> Self {
        use Cell::*;
        Self { board: [[Empty; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    fn full(&self) -> bool {
        self.board.iter().flatten().all(|&cell| cell != Cell::Empty)
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

    fn draw(&self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE{
                print!("{}", self.board[y][x]);
                if x != BOARD_SIZE - 1 {
                    print!("|");
                };
            }
            println!();
            if y != BOARD_SIZE - 1 {
                println!("-{}", "+-".repeat(BOARD_SIZE - 1));
            };
        }
    }

    fn set_cell(&mut self, index: usize, cell: Cell) {
        let x = index % 3;
        let y = index / 3;
        self.board[y][x] = cell;
    }

    fn play(&mut self) {
        let mut turn = Cell::X;
        self.draw();
        while !self.full() {
            println!("It's {turn}'s turn");
            self.set_cell(
                get_num(Some("Enter a number between 1 and 9> "), Some(1..=9)).unwrap() - 1,
                turn,
            );
            println!();
            self.draw();
            let winner = self.winner();
            if winner != Cell::Empty {
                println!("{winner} wins!");
                return;
            }
            turn.swap();
        }
        println!("It's a cat's game!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full(){
        use Cell::*;
        let board = TicTacToe {
            board: [
                [X; 3],
                [O, X, O],
                [X, Empty, O]
            ]
        };
        assert!(!board.full());
        let board = TicTacToe {
            board: [[X; 3]; 3]
        };
        assert!(board.full());
        let board = TicTacToe {
            board: [[Empty; 3]; 3]
        };
        assert!(!board.full());
    }

    #[test]
    fn test_winner() {
        use Cell::*;
        // horizontal
        let board = TicTacToe {
            board: [
                [X, Empty, X],
                [X; 3],
                [Empty; 3]
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToe {
            board: [
                [O; 3],
                [Empty; 3],
                [Empty; 3]
            ]
        };
        assert_eq!(board.winner(), O);
        // vertical
        let board = TicTacToe {
            board: [
                [X, O, X],
                [X, Empty, X],
                [X, O, Empty],
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToe {
            board: [
                [X, O, Empty],
                [Empty, O, X],
                [X, O, Empty],
            ]
        };
        assert_eq!(board.winner(), O);
        // diagonal \
        let board = TicTacToe {
            board: [
                [X, O, O],
                [O, X, O],
                [O, O, X]
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToe {
            board: [
                [O, X, X],
                [X, O, X],
                [X, X, O]
            ]
        };
        assert_eq!(board.winner(), O);
        // diagonal /
        let board = TicTacToe {
            board: [
                [O, O, X],
                [O, X, O],
                [X, O, X]
            ]
        };
        assert_eq!(board.winner(), X);
        let board = TicTacToe {
            board: [
                [O, X, O],
                [X, O, X],
                [O, X, X]
            ]
        };
        assert_eq!(board.winner(), O);
        // no winner
        let board = TicTacToe {
            board: [
                [X, O, X],
                [X, O, X],
                [O, X, O]
            ]
        };
        assert_eq!(board.winner(), Empty);
        let board = TicTacToe {
            board: [[Empty; 3]; 3]
        };
        assert_eq!(board.winner(), Empty);
    }
    
}

fn get_num<T: PartialOrd<T> + std::str::FromStr>(prompt: Option<&str>, range: Option<std::ops::RangeInclusive<T>>) -> std::io::Result<T> {
    loop {
        if let Some(prompt) = prompt {
            print!("{}", prompt);
            std::io::stdout().flush()?;
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if let Ok(value) = input.parse::<T>() {
            if let Some(ref range) = range {
                if range.contains(&value) {
                    return Ok(value);
                }
            } else {
                return Ok(value);
            }
        } else {
            println!("Input is not a valid number");
        }
    }
}

fn main() {
    TicTacToe::new().play();
}
