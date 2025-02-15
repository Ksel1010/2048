use std::{collections::HashMap, fmt::Display, usize, vec};
use rand::{rng, Rng};

/// Size of the board. The board is a square of size N x N.
pub const N: usize = 4;
// Cell type unsigned int 
pub type Cell = u32;
// An empty cell is declared to contain 0
pub const EMPTY_CELL: Cell = 0;



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Board {
    cells: [[Cell; N]; N],
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// Implements prettry printing for the `Direction` enum.
// This is what is used when you use the `{}` format specifier in a `println!` macro.
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "↑",
                Direction::Down => "↓",
                Direction::Left => "←",
                Direction::Right => "→",
            }
        )
    }
}
impl Board {
    pub const fn new_empty() -> Board {
        Board { cells: [[EMPTY_CELL ;N]; N] }
    }
    pub const fn new(cells: [[Cell; N]; N]) -> Board {
        Board { cells }
    }

    pub fn generate_random(&mut self) -> Option<Board>{
        let mut empty_cells : Vec<(usize, usize)> = Vec::new();
        for line in 0..N{
            for column in 0..N{
                if self.value_at(line, column) == 0 {
                    empty_cells.push((line, column));
                }
            }
        }
        let n = empty_cells.len();
        if n == 0{
            None
        }else{
            self.generate_at(*empty_cells.get(rng().random_range(0..n)).unwrap());
            Some(*self)
        }
      
    }

    pub fn best_move(&mut self, projection_nb:u8) -> Direction{
        if projection_nb ==0{
            panic!("To determine the best move you should project to at least one move but you tried with 0 moves")
        }
        // HashMap with a vector of the last layer boards possible starting from this direction
        let mut last_layer :HashMap<Direction, Vec<Option<Board>>> = HashMap::new();
        // a couple (Direction, max) to keep the best Direction and its "score"
        let mut max:(Direction, u32) = (Direction::Up, self.evaluate());
        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right]{
            let mut vector_board = Vec::new();
            // Inserting the first move in each (K, V)
            vector_board.push(self.play(direction));
            last_layer.insert(direction, vector_board);
            
            // for every Some(board) we found  
            for i in 0..projection_nb-1{
                let mut new_level_board: Vec<Option<Board>> = Vec::new();
            // we play every possible direction
                for post_direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right]{
                    for board_iterator in last_layer.get(&direction).unwrap(){
            // and we put the result in the new level boards
                        if let Some(mut board) = board_iterator {
                            new_level_board.push(board.play(post_direction));
                        }
                    }
                }
            // then we replace the result by the new level boards
            last_layer.insert(direction, new_level_board);
            }

            // check the max score of the last layer and if it is the max of all time update max(Direction, u32)
            for board_iterator in last_layer.get(&direction).unwrap(){
                if let Some(board) = board_iterator{
                    let score = board.evaluate();
                    if score > max.1 {
                        max = (direction, score);
                    }
                }
            }
        }
        // return the direction found
        if max.1 == self.evaluate(){
            panic!("You lost ! ")
        }else{
            max.0
        }
    
    }

    pub fn play(&mut self, direction: Direction) -> Option<Board>{
        let mut new_board = self.move_direction(direction);
        new_board.generate_random()
    }

    
    fn move_direction(&mut self, direction: Direction)-> Board{
        let mut new_board = Board::new_empty();
        match direction {
            Direction::Up =>{
                for column in 0..N {
                    let mut value_before: u32 = 0;
                    let mut new_cnt = 0;
                    for old_column_cnt in 0..N{
                        let val = self.value_at(old_column_cnt, column);
                        if val!=0{
                            if value_before==val{
                                new_board.cells[new_cnt-1][column] *= 2;
                                value_before = 0;
                            }else{
                                new_board.cells[new_cnt][column] = val;
                                value_before = val;
                                new_cnt += 1;
                            }
                        }
                    }
                }
                new_board
            },
            Direction::Left =>{
                for line in 0..N {
                    let mut value_before: u32 = 0;
                    let mut new_cnt = 0;
                    for old_line_cnt in 0..N{
                        let val = self.value_at(line, old_line_cnt);
                        if val!=0{
                            if value_before==val{
                                new_board.cells[line][new_cnt-1] *= 2;
                                value_before = 0;
                            }else{
                                new_board.cells[line][new_cnt] = val;
                                value_before = val;
                                new_cnt += 1;
                            }
                        }
                    }
                }
                new_board
            },
            Direction::Right=>{
                for line in 0..N {
                    let mut value_before: u32 = 0;
                    let mut new_cnt = N-1;
                    for old_line_cnt in (0..N).rev(){
                        let val = self.value_at(line, old_line_cnt);
                        if val!=0{
                            if value_before==val{
                                new_board.cells[line][new_cnt+1] *= 2;
                                value_before = 0;
                            }else{
                                new_board.cells[line][new_cnt] = val;
                                value_before = val;
                                new_cnt -= 1;
                            }
                        }
                    }
                }
                new_board
            },
            Direction::Down =>{
                for column in 0..N {
                    let mut value_before: u32 = 0;
                    let mut new_cnt = N-1;
                    for old_column_cnt in (0..N).rev(){
                        let val = self.value_at(old_column_cnt, column);
                        if val!=0{
                            if value_before==val{
                                new_board.cells[new_cnt+1][column] *= 2;
                                value_before = 0;
                            }else{
                                new_board.cells[new_cnt][column] = val;
                                value_before = val;
                                new_cnt -= 1;
                            }
                        }
                    }
                }
                new_board
            }
        }
    }

    pub fn evaluate(&self) -> u32 {
        let mut sum = 0;
        for i in 0..N{
            for j in 0..N{
                match self.value_at(i, j){
                    0 => sum +=1,
                    n => sum += n*n,
                }
            }
        }
        sum
    }
    
    fn generate_at(&mut self, coordinates:(usize, usize)){
        if rng().random_range(0..10)==0 {
            self.set(coordinates.0, coordinates.1, 4);
        }else{
            self.set(coordinates.0, coordinates.1, 2);
        }
    }

    pub fn value_at(&self, line: usize, column: usize)-> u32{
        self.cells[line][column]
    }

    pub fn set(&mut self, line: usize, column:usize, value: u32){
        self.cells[line][column] = value;
    }
}


/*impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n┏━━━┳━━━┳━━━┳━━━┓\n")?;
        for i in 0..N {
            write!(f, "┃")?;
            for j in 0..N {
                let value_in_cell = self.value_at(i, j);
                if value_in_cell == 0 {
                    write!(f, "   ┃")?;
                } else {
                    write!(f, " {value_in_cell} ┃")?;
                }
            }
            if i < N - 1 {
                write!(f, "\n┣━━━╋━━━╋━━━╋━━━┫\n")?;
            } else {
                write!(f, "\n┗━━━┻━━━┻━━━┻━━━┛\n")?;
            }
        }
        Ok(())
    }
}*/
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const CELL_WIDTH: usize = 5; // Fixed width for each cell

        // Generate top border dynamically
        write!(f, "\n┏{}┓\n", "━━━━━┳".repeat(N - 1) + "━━━━━")?;

        for i in 0..N {
            write!(f, "┃")?;
            for j in 0..N {
                let value_in_cell = self.value_at(i, j);
                if value_in_cell == 0 {
                    write!(f, " {:width$} ┃", "", width = CELL_WIDTH - 2)?; // Empty cell
                } else {
                    write!(f, " {:>width$} ┃", value_in_cell, width = CELL_WIDTH - 2)?; // Right-aligned number
                }
            }

            // Print row separators or bottom border
            if i < N - 1 {
                write!(f, "\n┣{}┫\n", "━━━━━╋".repeat(N - 1) + "━━━━━")?;
            } else {
                write!(f, "\n┗{}┛\n", "━━━━━┻".repeat(N - 1) + "━━━━━")?;
            }
        }
        Ok(())
    }
}





#[cfg(test)]
mod tests {
    // import everything from the containing module (Board, Direction, ...)
    use super::*;

    // A unit test that succeeds if the code does not panic.
    // This one is meant to test the indexing of the board
    #[test]
    fn test_move_direction(){
        let start = std::time::Instant::now();
        let mut initial_board = Board::new([  [0, 0, 2, 0],
                                                            [2, 0, 4, 0],
                                                            [2, 2, 0, 0],
                                                            [0, 2, 0, 2]]); 

        assert_eq!(initial_board.move_direction(Direction::Right), Board::new([[0 ,0, 0, 2],
                                                                     [0, 0, 2, 4],
                                                                     [0, 0, 0, 4],
                                                                     [0, 0, 0, 4]]));

        assert_eq!(initial_board.move_direction(Direction::Left), Board::new([ [2, 0, 0, 0],
                                                                     [2, 4, 0, 0],
                                                                     [4, 0, 0, 0],
                                                                     [4, 0, 0, 0]]));

        assert_eq!(initial_board.move_direction(Direction::Up), Board::new([  [4, 4, 2, 2],
                                                                    [0, 0, 4, 0],
                                                                    [0, 0, 0, 0],
                                                                    [0, 0, 0, 0]]));

        assert_eq!(initial_board.move_direction(Direction::Down), Board::new([[0, 0, 0, 0],
                                                                    [0, 0, 0, 0],
                                                                    [0, 0, 2, 0],
                                                                    [4, 4, 4, 2]]));

        // Second board
        initial_board = Board::new([  [8, 0, 2, 0],
                                             [2, 16, 4, 0],
                                             [2, 4, 4, 0],
                                             [0, 4, 0, 2]]);
        assert_eq!(initial_board.move_direction(Direction::Right), Board::new([[0 ,0, 8, 2],
                                                [0, 2, 16, 4],
                                                [0, 0, 2, 8],
                                                [0, 0, 4, 2]]));
        
        assert_eq!(initial_board.move_direction(Direction::Up), Board::new([ [8, 16, 2, 2],
                                                [4, 8, 8, 0],
                                                [0, 0, 0, 0],
                                                [0, 0, 0, 0]]));
            
        assert_eq!(initial_board.move_direction(Direction::Left), Board::new([  [8, 2, 0, 0],
                                               [2, 16, 4, 0],
                                               [2, 8, 0, 0],
                                               [4, 2, 0, 0]]));

        assert_eq!(initial_board.move_direction(Direction::Down), Board::new([[0, 0, 0, 0],
                                               [0, 0, 0, 0],
                                               [8, 16, 2, 0],
                                               [4, 8, 8, 2]])); 
                                                                 
        let runtime = start.elapsed();
        println!("runtime = {}", runtime.as_micros());
    } 
}