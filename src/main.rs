use std::collections::HashMap;
use reqwest::Error;
use crate::crossword::{Crossword, word};
use crate::Square::{CharSquare, StartSquare};
use reqwest::header::CONTENT_TYPE;
use reqwest::header::ACCEPT;


use std::io::{self};
use serde::Deserialize;
use crate::tarkovapi::{Ammo, new_tarkov_api};

mod crossword;
mod tarkovapi;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, world!");
    println!("Creating Crossword");
    let c = crossword::build_crossword();

    let crossword_size = c.get_size();
    let map = create_map(&c);

   let response = call().await;
    match response {
        Ok(r) => { println!("got response {:?}", r)}
        Err(e) => {
            println!("got error")
        }
    }

    Ok(())

}

async fn call() -> Result<Vec<Ammo>, Error> {
    let api = new_tarkov_api();

    let response = api.get_ammo().await?;
    Ok(response)
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