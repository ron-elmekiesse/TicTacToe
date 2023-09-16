pub mod board;

const PLAY_AGAIN_SYMBOL: &str = "y";

fn is_winner_in_rows(board: &board::Board, player_symbol: char) -> bool
{
    'rows_loop: for row in 0..board::ROWS
    {
        for col in 0..board::COLS
        {
            if board.get(row * board::ROWS + col) != player_symbol
            {
                continue 'rows_loop;
            }
        }
        
        return true;
    }

    return false;
}

fn is_winner_in_cols(board: &board::Board, player_symbol: char) -> bool
{
    'cols_loop: for col in 0..board::COLS
    {
        for row in 0..board::ROWS
        {
            if board.get(row * board::ROWS + col) != player_symbol
            {
                continue 'cols_loop;
            }
        }

        return true
    }

    return false;
}

fn is_winner_in_left_diagonal(board: &board::Board, player_symbol: char) -> bool
{
    // Check left to right
    let (mut row, mut col): (usize, usize) = (0, 0);

    while (row < board::ROWS) && (col < board::COLS)
    {
        if board.get(row * board::ROWS + col) != player_symbol
        {
            return false;
        }

        row += 1;
        col += 1;
    }
    
    return true;
}

fn is_winner_in_right_diagonal(board: &board::Board, player_symbol: char) -> bool
{
    // Check right to left
    let mut row: usize = 0;
    let mut col: usize = board::COLS - 1;

    while row < board::ROWS
    {
        if board.get(row * board::ROWS + col) != player_symbol
        {
            return false;
        }

        if 0 == col
        {
            break;
        }

        row += 1;
        col -= 1;
    }
    
    return true;
}

fn is_winner_in_diagonal(board: &board::Board, player_symbol: char) -> bool
{
   return is_winner_in_left_diagonal(board, player_symbol) ||
          is_winner_in_right_diagonal(board, player_symbol);
}

pub fn is_winner(board: &board::Board, player_symbol: char) -> bool
{
    let did_won: bool =  is_winner_in_rows(board, player_symbol) ||
                         is_winner_in_cols(board, player_symbol) ||
                         is_winner_in_diagonal(board, player_symbol);
    
    if did_won
    {
        println!("Congratulations! {} Won !", player_symbol);
        return true;
    }

    return false;
}

pub fn is_draw(board: &board::Board) -> bool
{
    let locations_range = board::LOCATIONS_RANGE;
    for location in locations_range
    {
        if board.get(location - 1) == (((location as u8) + b'0') as char)
        {
            return false;
        }
    }

    println!("Game has finished in a draw!");

    return true;
}

pub fn insert_player_choice(board: &mut board::Board, choice: &String, symbol: char) -> bool
{
    let location: usize = choice.parse().unwrap();

    if !board::LOCATIONS_RANGE.contains(&location) || !board.empty(location - 1)
    {
        return false;
    }

    return board.insert(location - 1, symbol);
}

pub fn get_user_input() -> String
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Invalid user input!");
    return String::from(input.trim());
}

pub fn does_user_play_again() -> bool
{
    println!("Enter 'y' to play again or else to stop: ");

    let input = get_user_input();
    return PLAY_AGAIN_SYMBOL == input;
}