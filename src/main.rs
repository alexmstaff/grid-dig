use std::io;

use console::{Key, Term};

const BOARD_SIZE: usize = 16;
const BLANK_SQUARE: char = ' ';

fn main() {
    let mut board = Board::new();
    let mut player = Player::new();
    game_loop(&mut board, &mut player).unwrap();
    println!("Thanks for playing!")
}

fn game_loop(board: &mut Board, player: &mut Player) -> io::Result<()> {
    let term = Term::stdout();

    term.hide_cursor()?;
    term.clear_screen()?;

    println!("Use arrow keys to controll digger.");
    println!("Esc ends the game.");
    println!("Press any key to start");

    term.read_key()?;

    loop {
        term.clear_screen()?;

        let (x, y) = player.location.get_loc();
        board.vector[y][x] = player.symbol;
        print_board(&board.vector);

        let user_move = term.read_key()?;
        match user_move {
            Key::ArrowUp => {
                move_player(player, board, (0, -1));
            }
            Key::ArrowDown => {
                move_player(player, board, (0, 1));
            }
            Key::ArrowLeft => {
                move_player(player, board, (-1, 0));
            }
            Key::ArrowRight => {
                move_player(player, board, (1, 0));
            }
            Key::Escape => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
    term.clear_screen()?;
    term.show_cursor()?;

    Ok(())
}

fn move_player(player: &mut Player, board: &mut Board, target: (i8, i8)) {
    {
        let player_loc = player.location.get_loc();
        let target_square = board.get_cell(BoardLoc::location_from_target(&player_loc, target));

        match target_square {
            ' ' => {
                board.vector[player_loc.1][player_loc.0] = BLANK_SQUARE;
                player.set_loc(BoardLoc::location_from_target(
                    &player.location.get_loc(),
                    target,
                ));
            }
            _ => return,
        }
    };
}

fn build_board() -> Vec<Vec<char>> {
    let vert_wall = '|';
    let hor_wall = '#';

    let top_and_bottom = vec![hor_wall; BOARD_SIZE + 2];
    let mut board = Vec::with_capacity(BOARD_SIZE + 2);

    board.push(top_and_bottom);

    for _ in 0..BOARD_SIZE {
        let mut row = Vec::with_capacity(BOARD_SIZE + 2);
        row.push(vert_wall);
        for _ in 0..BOARD_SIZE {
            row.push(BLANK_SQUARE);
        }
        row.push(vert_wall);
        board.push(row);
    }
    let top_and_bottom = vec![hor_wall; BOARD_SIZE + 2];

    board.push(top_and_bottom);

    board
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in 0..board.len() {
        for cell in 0..board[row].len() {
            print!("{}", board[row][cell])
        }
        println!()
    }
}

fn add_target_to_loc(u: usize, i: i8) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as usize)
    } else {
        u.checked_add(i as usize)
    }
}

struct Player {
    location: BoardLoc,
    symbol: char,
}

impl Player {
    fn new() -> Player {
        Player {
            location: BoardLoc {
                x: BOARD_SIZE / 2,
                y: 2,
            },
            symbol: '@',
        }
    }

    fn set_loc(&mut self, player_loc: BoardLoc) {
        self.location.x = player_loc.x;
        self.location.y = player_loc.y;
    }
}

struct Board {
    vector: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Board {
        Board {
            vector: build_board(),
        }
    }

    fn get_cell(&self, player_loc: BoardLoc) -> char {
        self.vector[player_loc.y][player_loc.x]
    }
}

struct BoardLoc {
    x: usize,
    y: usize,
}

impl BoardLoc {
    fn location_from_target(old_loc: &(usize, usize), target: (i8, i8)) -> BoardLoc {
        BoardLoc {
            x: add_target_to_loc(old_loc.0, target.0).unwrap(),
            y: add_target_to_loc(old_loc.1, target.1).unwrap(),
        }
    }

    fn get_loc(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
