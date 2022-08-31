use console::style;
use rand::Rng;

const BOARD_SIZE: usize = 24;
const BLANK_SQUARE: char = ' ';
const RESOURCE_SQUARE: char = 'ↈ';
const DUG_SQUARE: char = '.';

pub fn move_player(player: &mut Player, board: &mut Board, target: (i8, i8)) {
    let player_loc = player.location.get_loc();
    let target_square = board.get_cell(&BoardLoc::location_from_target(&player_loc, target));

    match target_square {
        BLANK_SQUARE | RESOURCE_SQUARE | DUG_SQUARE => {
            board.vector[player_loc.1][player_loc.0] = BLANK_SQUARE;
            player.set_loc(BoardLoc::location_from_target(
                &player.location.get_loc(),
                target,
            ));
        }
        _ => return,
    }
}

pub fn debris_sim(block_loc: (usize, usize), board: &mut Board) -> Block {
    let target_loc = BoardLoc::location_from_target(&block_loc, (0, 1));
    let target_square = board.get_cell(&target_loc);

    match target_square {
        BLANK_SQUARE => {
            board.vector[block_loc.1][block_loc.0] = BLANK_SQUARE;
            return Block::new(target_loc, '.');
        }
        _ => {
            return Block::new(
                BoardLoc {
                    x: block_loc.0,
                    y: block_loc.1,
                },
                '.',
            )
        }
    }
}

fn build_board_vector() -> (Vec<Vec<char>>, Vec<Block>) {
    let vert_wall = '|';
    let hor_wall = '#';

    let top_and_bottom = vec![hor_wall; 3 * BOARD_SIZE + 2];
    let mut board = Vec::with_capacity(BOARD_SIZE + 2);
    let mut blocks = Vec::with_capacity(BOARD_SIZE);

    board.push(top_and_bottom);

    for y in 1..=BOARD_SIZE {
        let mut row = Vec::with_capacity(3 * BOARD_SIZE + 2);
        row.push(vert_wall);
        for x in 1..=3 * BOARD_SIZE {
            let block_type = rand::thread_rng().gen_range(0..=10);
            if block_type < 4 {
                row.push(BLANK_SQUARE);
                blocks.push(Block::build((x, y)));
            } else {
                row.push(BLANK_SQUARE);
            }
        }
        row.push(vert_wall);
        board.push(row);
    }

    let top_and_bottom = vec![hor_wall; 3 * BOARD_SIZE + 2];
    board.push(top_and_bottom);

    (board, blocks)
}

pub fn print_board(board: &Board) {
    let board = &board.vector;
    for row in 0..board.len() {
        for cell in 0..board[row].len() {
            match board[row][cell] {
                '#' | '|' => {
                    print!("{}", style(board[row][cell]).cyan())
                }
                RESOURCE_SQUARE => {
                    print!("{}", style(board[row][cell]).yellow())
                }
                DUG_SQUARE => {
                    print!("{}", style(board[row][cell]).red())
                }
                '@' => {
                    print!("{}", style(board[row][cell]).green())
                }
                _ => {
                    print!("{}", (board[row][cell]))
                }
            }
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

pub struct Player {
    location: BoardLoc,
    symbol: char,
}

impl Player {
    pub fn new() -> Player {
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

    pub fn get_loc(&self) -> (usize, usize) {
        self.location.get_loc()
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }
}

pub struct Board {
    vector: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> (Board, Vec<Block>) {
        let (vector, blocks) = build_board_vector();
        (Board { vector }, blocks)
    }

    fn get_cell(&self, player_loc: &BoardLoc) -> char {
        self.vector[player_loc.y][player_loc.x]
    }

    pub fn set_cell(&mut self, target: (usize, usize), symbol: char) {
        self.vector[target.1][target.0] = symbol;
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

pub struct Block {
    location: BoardLoc,
    symbol: char,
}

impl Block {
    fn new(location: BoardLoc, symbol: char) -> Block {
        Block { location, symbol }
    }

    fn build((x, y): (usize, usize)) -> Block {
        let symbols = vec!['█', '▓', RESOURCE_SQUARE];
        let symbol = symbols[rand::thread_rng().gen_range(0..=2)];
        Block::new(BoardLoc { x, y }, symbol)
    }

    pub fn digg((x, y): (usize, usize)) -> Block {
        Block::new(BoardLoc { x, y }, DUG_SQUARE)
    }

    pub fn get_loc(&self) -> (usize, usize) {
        self.location.get_loc()
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }
}
