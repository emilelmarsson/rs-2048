use rand::prelude::*;

enum Direction{
    Left,
    Up,
    Right,
    Down
}

const BOARD_SIZE: usize = 4;

#[derive(Debug)]
struct Board{
    board: [Option<Tile>; BOARD_SIZE * BOARD_SIZE],
}

impl Board{
    fn sum_rows(&mut self, left: bool){
        for i in 0..BOARD_SIZE{
            let mut non_empty_tiles: Vec<Tile> = Vec::new();
            let mut merge = true;
            let mut count = 0;
            for j in if left { 0..BOARD_SIZE } else { BOARD_SIZE..0 }{
                println!("HEJJJJJ{}", j);
                if let Some(t) = self.board[i * BOARD_SIZE + j]{
                    if merge && count > 0 && non_empty_tiles[count-1].val == t.val{
                        non_empty_tiles[count-1].val += t.val;
                        merge = false;
                    } else {
                        non_empty_tiles.push(t);
                        count+=1;
                        merge = true;
                    }

                    let offset = if left { count - 1 } else { BOARD_SIZE - count };
    
                    self.board[i * BOARD_SIZE + offset ] = Some(non_empty_tiles[count - 1]);
                    if offset != j {
                        self.board[i * BOARD_SIZE + j] = None;
                    }
                }
            }
        }
    }

    fn swipe(&mut self, direction: Direction){
        match direction{
            Direction::Left => {
               self.sum_rows(true);
            },
            Direction::Up => {

            },
            Direction::Right => {
                self.sum_rows(false);
            },
            Direction::Down => {

            },
        }
    }

    fn new() -> Board{
        let mut board = [None; BOARD_SIZE * BOARD_SIZE];

        let mut rng = rand::thread_rng();

        for _i in 0..rng.gen_range(8..11){ // Start with 1 or 2 tiles.
            // Randomly select a starting tile
            let empty_tile_index = board.iter()
                                        .enumerate()
                                        .filter(|(_i,t)| t.is_none())
                                        .choose(&mut rng)
                                        .unwrap()
                                        .0;

            board[empty_tile_index] = Some(Tile {val: rng.gen_range(1..3) * 2});
        }

        Board { board: board }
    }
}

#[derive(Debug, Copy, Clone)]
struct Tile{
    val: u32,
}

fn main(){
    let mut board = Board::new();

    for i in 0..board.board.len(){
        match board.board[i] {
            None => print!("- "),
            Some(t) => print!("{} ", t.val),
        }

        if i % 4 == 3 { println!() };
    }

    println!();

    board.swipe(Direction::Right);

    for i in 0..board.board.len(){
        match board.board[i] {
            None => print!("- "),
            Some(t) => print!("{} ", t.val),
        }

        if i % 4 == 3 { println!() };
    }

    //println!("{:?}", board);
}