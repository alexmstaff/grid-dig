use std::io;

use console::{Key, Term};

const BOARD_SIZE: usize = 16;
const BLANK_SQUARE: char = ' ';

struct Player {
    x: usize,
    y: usize,
    symbol: char,
}

impl Player {
    fn new() -> Player {
        Player {
            x: 10,
            y: 10,
            symbol: '@',
        }
    }
}

fn main() {
    let mut board = build_board();
    let mut player = Player::new();
    game_loop(&mut board, &mut player).unwrap();
    println!("Thanks for playing!")
}

fn game_loop(board: &mut Vec<Vec<char>>, player: &mut Player) -> io::Result<()> {
    let term = Term::stdout();

    term.hide_cursor()?;
    term.clear_screen()?;

    println!("Use arrow keys to controll digger.");
    println!("Esc ends the game.");
    println!("Press any key to start");

    term.read_key()?;

    loop {
        term.clear_screen()?;

        board[player.y][player.x] = player.symbol;
        print_board(&board);

        let user_move = term.read_key()?;
        match user_move {
            Key::ArrowUp => {
                board[player.y][player.x] = BLANK_SQUARE;
                player.y -= 1;
            }
            Key::ArrowDown => {
                board[player.y][player.x] = BLANK_SQUARE;
                player.y += 1;
            }
            Key::ArrowLeft => {
                board[player.y][player.x] = BLANK_SQUARE;
                player.x -= 1;
            }
            Key::ArrowRight => {
                board[player.y][player.x] = BLANK_SQUARE;
                player.x += 1;
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

// fn move_player()

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
    for row in 0..BOARD_SIZE + 2 {
        for cell in 0..BOARD_SIZE + 2 {
            print!("{}", board[row][cell])
        }
        println!()
    }
}
