use std::fmt;
use std::io;
use std::io::Read;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Alignment};
use tui::style::{Color, Style, Modifier};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

#[derive(PartialEq)]
enum Dir{
    Left,
    Up,
    Right,
    Down
}

const EMPTY: u32 = 0;
const BOARD_SIZE: usize = 4;

#[derive(Debug)]
struct Board{
    board: [[u32; BOARD_SIZE]; BOARD_SIZE],
}

impl Board{
    const colors: [Color; 11] = [
        Color::Rgb(238, 228, 218),
        Color::Rgb(237, 224, 200),
        Color::Rgb(242, 177, 121),
        Color::Rgb(245, 149, 99),
        Color::Rgb(246, 124, 95),
        Color::Rgb(246, 94 ,59),
        Color::Rgb(237, 207, 114),
        Color::Rgb(237, 204, 97),
        Color::Rgb(237, 200, 80),
        Color::Rgb(237, 197, 63),
        Color::Rgb(237, 194, 46),
    ];

    fn set_column(&mut self, col_index: usize, column: &[u32; 4]){
        for i in 0..BOARD_SIZE{
            self.board[i][col_index] = column[i];
        }
    }

    fn swipe(&mut self, dir: Dir){
        for i in 0..BOARD_SIZE{
            let mut new_row = [0u32; 4];
            let mut counter = 0;
            let mut merge = true;

            for j in 0..BOARD_SIZE{
                let cur_tile = 
                    match dir{
                        Dir::Left => self.board[i][j],
                        Dir::Up => self.board[j][i],
                        Dir::Right => self.board[i][BOARD_SIZE - 1 - j],
                        Dir::Down => self.board[BOARD_SIZE - 1 - j][i],
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
                    self.board[i] = new_row;
                },
                Dir::Up => {
                    self.set_column(i, &new_row);
                },
                Dir::Right => {
                    new_row.reverse();
                    self.board[i] = new_row;
                },
                Dir::Down => {
                    new_row.reverse();
                    self.set_column(i, &new_row);
                },
            };
        }
    }

    fn new() -> Board{
        let mut board = [[0u32; BOARD_SIZE]; BOARD_SIZE];

        board[0] = [2, 2, 4, 2];
        board[1] = [4, 0, 0, 4];
        board[2] = [2, 2, 2, 2];
        board[3] = [0, 4, 2, 0];

        Board { board: board }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..BOARD_SIZE{
            for j in 0..BOARD_SIZE{
                if self.board[i][j] == EMPTY{
                    write!(f, "- ");
                } else {
                    write!(f, "{} ", self.board[i][j]);
                }
            }
    
            write!(f, "\n");
        }

        Ok(())
    }
}

/*fn main(){
    let mut board = Board::new();

    loop{
        println!("{}", board);
        
        let mut line = String::new();
        println!("Enter direction (U, R, D, L):");

        std::io::stdin().read_line(&mut line).unwrap();
        match line.to_uppercase().trim() {
            "U" => {board.swipe(Direction::Up)},
            "R" => {board.swipe(Direction::Right)},
            "D" => {board.swipe(Direction::Down)},
            "L" => {board.swipe(Direction::Left)},
            _ => ()
        }
    }
}*/

fn main() -> Result<(), io::Error> {
    let mut board = Board::new();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut asi = async_stdin();

    terminal.clear()?;
    loop {
        terminal.draw(|frame| {
            let row_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            for i in 0..4{
                let col_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(25),
                            Constraint::Percentage(25),
                            Constraint::Percentage(25),
                            Constraint::Percentage(25),
                        ]
                        .as_ref(),
                    )
                    .split(row_chunks[i]);

                for j in 0..4{
                    let tile = board.board[j][i];
                    let tile_string: String = if tile == 0 { String::new() } else { tile.to_string() };
                    let bg_color: Color = 
                        if tile == 2 { Color::Rgb(238, 228, 218) } 
                        else if tile == 4 { Color::Rgb(237, 224, 200) }
                        else { Color::Black };

                    let paragraph = 
                        Paragraph::new(tile_string)
                        .block(Block::default().borders(Borders::ALL).style(Style::default().bg(bg_color)))
                        .style(Style::default().add_modifier(Modifier::BOLD))
                        .alignment(Alignment::Center);

                    frame.render_widget(paragraph, col_chunks[j]);
                }
            }
        })?;

        for k in asi.by_ref().keys() {
            match k.unwrap() {
                Key::Left => { board.swipe(Dir::Left); },
                Key::Right => { board.swipe(Dir::Right);},
                Key::Up => { board.swipe(Dir::Up); },
                Key::Down => { board.swipe(Dir::Down); },
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                _ => (),
            }
        }
    }
}