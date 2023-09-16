mod game_utils;
use game_utils::board as board;

const X_PLAYER_SYMBOL: char = 'X';
const Y_PLAYER_SYMBOL: char = 'Y';

macro_rules!  SWITCH_CURRENT_PLAYER_SYMBOL {
    ($a:expr) => {
        if X_PLAYER_SYMBOL == $a { Y_PLAYER_SYMBOL } else { X_PLAYER_SYMBOL }
    };
}

fn make_user_turn(game_board: &mut board::Board, current_turn_symbol: char) -> bool
{
    let current_player_choice = game_utils::get_user_input();

    return game_utils::insert_player_choice(game_board, &current_player_choice, current_turn_symbol);
}

fn play_single_game() -> ()
{
    let mut game_board = board::Board::new();
    let mut current_turn_symbol: char = X_PLAYER_SYMBOL;
    let mut is_game_done: bool = false;

    while !is_game_done
    {
        game_board.display();

        println!("It's {} turn:", current_turn_symbol);

        loop
        {
            if make_user_turn(&mut game_board, current_turn_symbol)
            {
                break;
            }
            
            println!("Invalid choice!");
        }

        is_game_done = game_utils::is_winner(&game_board, current_turn_symbol) ||
                       game_utils::is_draw(&game_board);

        current_turn_symbol = SWITCH_CURRENT_PLAYER_SYMBOL!(current_turn_symbol);
    }

    game_board.display();

}

pub fn play() -> ()
{
    loop
    {
        play_single_game();
        
        if !game_utils::does_user_play_again()
        {
            break;
        }
    }

    println!("Thank you for playing. ~R.E");
}