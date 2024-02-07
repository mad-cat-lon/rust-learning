use core::fmt;
use std::{io::Write, process};

const ROWS: usize = 3;
const COLS: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    X,
    O,
}

#[derive(Clone, Copy)]
struct Square {
    piece: Option<Piece>,
}

impl Square {
    pub fn new() -> Square {
        Square { piece: None }
    }

    fn symbol(&self) -> &str {
        match self.piece {
            Some(Piece::X) => "X",
            Some(Piece::O) => "O",
            None => ".",
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

struct Board {
    current_turn: i64,
    squares: [[Square; COLS]; ROWS],
}

impl Board {
    pub fn new() -> Board {
        let squares = [[Square::new(); COLS]; ROWS];
        Board {
            current_turn: 1,
            squares: squares,
        }
    }

    pub fn place(&mut self, piece: Piece, x: usize, y: usize) -> Result<(), &'static str> {
        let turn = ((self.current_turn % 2) + 2) % 2;
        if x >= COLS || y >= ROWS {
            Err("Position is out of bounds")
        } else if self.squares[x][y].piece.is_some() {
            Err("Position is already occupied")
        } else if turn == 0 && piece != Piece::X {
            Err("It's X's turn!")
        } else if turn == 1 && piece != Piece::O {
            Err("It's O's turn!")
        } else {
            self.squares[x][y].piece = Some(piece);
            self.current_turn += 1;
            Ok(())
        }
    }

    pub fn check_state(&mut self) -> Option<Piece> {
        // Check for winning condition in rows
        for row in &self.squares {
            if row.iter().all(|square| square.piece == Some(Piece::O)) {
                return Some(Piece::O);
            } else if row.iter().all(|square| square.piece == Some(Piece::X)) {
                return Some(Piece::X);
            }
        }

        // Check columns
        for col in 0..COLS {
            if (0..ROWS).all(|row| self.squares[row][col].piece == Some(Piece::O)) {
                return Some(Piece::O);
            } else if (0..ROWS).all(|row| self.squares[row][col].piece == Some(Piece::X)) {
                return Some(Piece::X);
            }
        }

        // Check diagonals
        if (0..ROWS).all(|i| self.squares[i][i].piece == Some(Piece::O))
            || (0..ROWS).all(|i| self.squares[i][ROWS - i - 1].piece == Some(Piece::O))
        {
            return Some(Piece::O);
        } else if (0..ROWS).all(|i| self.squares[i][i].piece == Some(Piece::X))
            || (0..ROWS).all(|i| self.squares[i][ROWS - i - 1].piece == Some(Piece::X))
        {
            return Some(Piece::X);
        }

        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        let top = "-".repeat(15);
        res.push_str(top.as_str());
        res.push_str("\n");
        for i in 0..ROWS {
            for j in 0..COLS {
                res.push_str("| ");
                res.push_str(self.squares[i][j].symbol());
                res.push_str(" |");
            }
            res.push_str("\n");
            let bot = "-".repeat(15);
            res.push_str(bot.as_str());
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}

fn read_input() -> Vec<String> {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Cannot read input");
    let args = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    args
}

fn main() {
    println!("Game started!");
    let mut board = Board::new();
    println!("{}", board);
    let mut state = board.check_state();
    while state == None {
        let args = read_input();
        let piece_string = &args[0];
        match piece_string.as_str() {
            "X" => {
                let x: usize = args[1].parse().unwrap_or_else(|e| {
                    println!("Failed to parse x {}", e);
                    process::exit(1);
                });
                let y: usize = args[2].parse().unwrap_or_else(|e| {
                    println!("Failed to parse y {}", e);
                    process::exit(1);
                });
                let piece = Piece::X;
                let res = board.place(piece, x, y);
                match res {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
            }
            "O" => {
                let x: usize = args[1].parse().unwrap_or_else(|e| {
                    println!("Failed to parse x {}", e);
                    process::exit(1);
                });
                let y: usize = args[2].parse().unwrap_or_else(|e| {
                    println!("Failed to parse y {}", e);
                    process::exit(1);
                });
                let piece = Piece::O;
                let res = board.place(piece, x, y);
                match res {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
            }
            _ => {
                println!("Command format: X/O row col")
            }
        }
        println!("{}", board);
        state = board.check_state();
    }
    if state == Some(Piece::O) {
        println!("O wins!");
        process::exit(0)
    } else {
        println!("X wins!");
        process::exit(0)
    }
}
