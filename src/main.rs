use std::collections::HashMap;
use crate::crossword::{Crossword, word};
use crate::Square::{CharSquare, StartSquare};

use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};

mod crossword;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    println!("Creating Crossword");
    let c = crossword::build_crossword();

    let crossword_size = c.get_size();
    let map = create_map(&c);

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())

}


fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new().constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])//.direction(Layout::Direction::Vertical)
        .split(frame.size());
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[2],
    );

    let inner_layout = Layout::new().constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).direction(Direction::Horizontal)
        .split(main_layout[1]);
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Left"),
        inner_layout[0],
    );
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Right"),
        inner_layout[1],
    );
}
fn create_map(crossword: &Crossword) -> HashMap<(u8,u8), Square> {
    let mut squares: HashMap<(u8,u8), Square> = HashMap::new();
    let words = crossword.get_words();
    words.iter().for_each(|w| {
        let i = w.get_i();
        let j = w.get_j();
        let square;
        if squares.contains_key(&(i, j)) {
            square = squares.get(&(i, j)).unwrap().add_orientation((w.get_orientation(), w.get_direction()))
        } else {
            square = StartSquare{i, j, c: w.get_word().chars().nth(0).unwrap(), orientation: vec![(w.get_orientation(), w.get_direction())]}
        }
        squares.insert((i, j), square);
        for x in 1..w.get_word().len() {
            if w.get_orientation() == word::Orientation::HORIZONTAL {
                let new_i;
                if w.get_direction() == word::Direction::DIRECT {
                    new_i = i+(x as u8);
                } else {
                    new_i = i-(x as u8);
                }
                squares.insert((new_i, j), CharSquare{i: new_i, j, c: w.get_word().chars().nth(x).unwrap()});
            } else {
                let new_j;
                if w.get_direction() == word::Direction::DIRECT {
                    new_j = j+(x as u8);
                } else {
                    new_j = j-(x as u8);
                }
                squares.insert((i, new_j), CharSquare{i, j: new_j, c: w.get_word().chars().nth(x).unwrap()});
            }

        }
    });

    return squares
}

#[derive(Debug, Clone)]
enum Square {
    CharSquare{i: u8, j: u8, c: char},
    StartSquare{i: u8, j: u8, c: char, orientation: Vec<(word::Orientation, word::Direction)>}
}

impl Square {
    fn add_orientation(&self, new_orientation: (word::Orientation, word::Direction)) -> crate::Square {
        return match self {
            CharSquare { i, j, c } => StartSquare{i: *i, j: *j, c: *c, orientation: vec![new_orientation] },
            StartSquare { i, j, c, orientation } => {
                let mut adding_orientation = vec![];
                adding_orientation.append(orientation.clone().as_mut());
                adding_orientation.push(new_orientation);
                StartSquare {i: *i, j: *j, c: *c, orientation: adding_orientation}
            }
        }
    }
}