pub const ROWS: usize = 3;
pub const COLS: usize = 3;
pub const BOARD_SIZE: usize = ROWS * COLS;
pub const LOCATIONS_RANGE: core::ops::Range<usize> = 1..BOARD_SIZE + 1;

pub struct Board
{
    board: [char; BOARD_SIZE]
}

impl Board
{
    pub fn new() -> Board
    {
        // Initialized a Board with the range

        let mut locations_range = LOCATIONS_RANGE;
        let locations: [char; BOARD_SIZE] = std::array::from_fn(|_|
            ((locations_range.next().expect("Invalid range given") as u8) + b'0') as char
        );

        return Board { board: locations };
    }

    pub fn empty(&self, index: usize) -> bool
    {
        return self.get(index) == ((index as u8 + 1) + b'0') as char;
    }

    pub fn insert(&mut self, index: usize, element: char) -> bool
    {
        if !LOCATIONS_RANGE.contains(&(index + 1))
        {
            return false;
        }

        self.board[index] = element;
        return true;
    }

    pub fn get(&self, index: usize) -> char
    {
        assert!(LOCATIONS_RANGE.contains(&(index + 1)));
        
        return self.board[index];
    }

    // TODO: std::fmt::Display
    pub fn display(&self) -> ()
    {
        for row in 0..ROWS
        {
            for col in 0..COLS
            {
                print!("{}", self.board[row * ROWS + col]);
                
                if COLS - 1 != col
                {
                    print!("|");
                }
            }

            println!("");
        }
    }
}