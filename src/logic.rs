const EMPTY: u32 = 0;
const BOARD_SIZE: usize = 4;

use std::fmt;

#[derive(PartialEq)]
pub enum Dir{
    Left,
    Up,
    Right,
    Down
}

#[derive(Debug)]
pub struct Board{
    pub tiles: [[u32; BOARD_SIZE]; BOARD_SIZE],

    current_move_index: usize,
    moves: Vec<[[u32; BOARD_SIZE]; BOARD_SIZE]>,
}

impl Board{
    fn undo(&mut self){
        
    }

    fn redo(&mut self){
        
    }

    fn set_column(&mut self, col_index: usize, column: &[u32; 4]){
        for i in 0..BOARD_SIZE{
            self.tiles[i][col_index] = column[i];
        }
    }

    pub fn swipe(&mut self, dir: Dir){
        for i in 0..BOARD_SIZE{
            let mut new_row = [0u32; 4];
            let mut counter = 0;
            let mut merge = true;

            for j in 0..BOARD_SIZE{
                let cur_tile = 
                    match dir{
                        Dir::Left => self.tiles[i][j],
                        Dir::Up => self.tiles[j][i],
                        Dir::Right => self.tiles[i][BOARD_SIZE - 1 - j],
                        Dir::Down => self.tiles[BOARD_SIZE - 1 - j][i],
                    };

                if cur_tile == EMPTY{
                    continue;
                }

                if merge && counter > 0 && cur_tile == new_row[counter - 1]{
                    new_row[counter - 1] *= 2;
                    merge = false;
                }else{
                    new_row[counter] = cur_tile;
                    merge = true;
                    counter += 1;
                }
            }

            match dir{
                Dir::Left => {
                    self.tiles[i] = new_row;
                },
                Dir::Up => {
                    self.set_column(i, &new_row);
                },
                Dir::Right => {
                    new_row.reverse();
                    self.tiles[i] = new_row;
                },
                Dir::Down => {
                    new_row.reverse();
                    self.set_column(i, &new_row);
                },
            };
        }
    }

    pub fn new() -> Board{
        let mut tiles = [[0u32; BOARD_SIZE]; BOARD_SIZE];

        tiles[0] = [2, 2, 4, 2];
        tiles[1] = [4, 0, 0, 4];
        tiles[2] = [2, 2, 2, 2];
        tiles[3] = [0, 4, 2, 0];

        Board { tiles: tiles }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..BOARD_SIZE{
            for j in 0..BOARD_SIZE{
                if self.tiles[i][j] == EMPTY{
                    write!(f, "- ");
                } else {
                    write!(f, "{} ", self.tiles[i][j]);
                }
            }
    
            write!(f, "\n");
        }

        Ok(())
    }
}