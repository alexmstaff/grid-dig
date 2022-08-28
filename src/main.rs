use std::io;

use console::{Key, Term};
use grid_dig::{move_player, print_board, Board, Player};

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

        let player_loc = player.get_loc();
        board.set_cell(player_loc, player.get_symbol());
        print_board(&board);

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
