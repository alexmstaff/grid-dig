const BOARD_SIZE: usize = 16;

fn main() {
    let board = build_board();
    print_board(board);
}

fn build_board() -> Vec<Vec<char>> {
    let vert_wall = '|';
    let hor_wall = '#';
    let blank_square = ' ';

    let top_and_bottom = vec![hor_wall; BOARD_SIZE + 2];
    let mut board = Vec::with_capacity(BOARD_SIZE + 2);

    board.push(top_and_bottom);

    for _ in 0..BOARD_SIZE {
        let mut row = Vec::with_capacity(BOARD_SIZE + 2);
        row.push(vert_wall);
        for _ in 0..BOARD_SIZE {
            row.push(blank_square);
        }
        row.push(vert_wall);
        board.push(row);
    }
    let top_and_bottom = vec![hor_wall; BOARD_SIZE + 2];

    board.push(top_and_bottom);

    board
}

fn print_board(board: Vec<Vec<char>>) {
    for row in 0..BOARD_SIZE + 2 {
        for cell in 0..BOARD_SIZE + 2 {
            print!("{}", board[row][cell])
        }
        println!()
    }
}
