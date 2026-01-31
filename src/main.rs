use std::io;

use console::{Key, Term};
use grid_dig::{
    debris_sim, move_player, print_board, Block, Board, Player, DUG_SQUARE, RESOURCE_SQUARE,
};

fn main() {
    let (board, blocks) = Board::new();
    let player = Player::new();
    game_loop(board, player, blocks).unwrap();
    println!("Thanks for playing!")
}

fn game_loop(mut board: Board, mut player: Player, mut blocks: Vec<Block>) -> io::Result<()> {
    let term = Term::stdout();

    term.hide_cursor()?;
    term.clear_screen()?;

    println!("Use arrow keys to control digger.");
    println!("Esc ends the game.");
    println!("Press any key to start");

    term.read_key()?;
    term.clear_screen()?;

    loop {
        let player_loc = player.get_loc();
        let block_count = blocks.len();
        let mut new_blocks = Vec::with_capacity(block_count);
        let dig_target = player.get_dig_target();

        for _ in 0..block_count {
            let block = blocks.pop().unwrap();
            let block_loc = block.get_loc();
            let block_symbol = block.get_symbol();

            if block_loc == player_loc && block_symbol == RESOURCE_SQUARE {
                new_blocks.push(Block::collect_resource(block_loc))
            } else if block_loc == dig_target {
                let dug_block = Block::dig(block_loc, block_symbol);
                board.set_cell(dug_block.get_loc(), dug_block.get_symbol());
                new_blocks.push(dug_block);
            } else {
                match block_symbol {
                    DUG_SQUARE => {
                        let debris_block = debris_sim(block_loc, &mut board);
                        board.set_cell(debris_block.get_loc(), debris_block.get_symbol());
                        new_blocks.push(debris_block);
                    }
                    _ => {
                        board.set_cell(block_loc, block_symbol);
                        new_blocks.push(block);
                    }
                }
            }
        }

        blocks.append(&mut new_blocks);

        board.set_cell(player_loc, player.get_symbol());

        print_board(&board);
        term.move_cursor_to(0, 0)?;

        let user_move = term.read_key()?;
        match user_move {
            Key::ArrowUp => {
                move_player(&mut player, &mut board, (0, -1));
            }
            Key::ArrowDown => {
                move_player(&mut player, &mut board, (0, 1));
            }
            Key::ArrowLeft => {
                move_player(&mut player, &mut board, (-1, 0));
            }
            Key::ArrowRight => {
                move_player(&mut player, &mut board, (1, 0));
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
