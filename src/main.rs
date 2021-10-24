mod logic;

use std::io;
use std::io::Read;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Alignment};
use tui::style::{Color, Style, Modifier};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

use logic::{Board, Dir};

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
                    let tile = board.tiles[j][i];
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