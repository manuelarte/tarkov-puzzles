use std::collections::HashMap;
use crate::crossword::Crossword;
use crate::crossword::word::{Direction, Orientation};
use crate::crossword::word::Orientation::HORIZONTAL;
use crate::crossword::word::Orientation::VERTICAL;
use crate::Square::{CharSquare, StartSquare};

mod crossword;

fn main() {
    println!("Hello, world!");
    println!("Creating Crossword");
    let c = crossword::build_crossword();

    let crossword_size = c.get_size();
    let map = create_map(&c);

    for j in 0..crossword_size.0 {
        for i in 0..crossword_size.1 {
            if map.contains_key(&(i, j)) {
                let square = map.get(&(i, j)).unwrap();
                match square {
                    CharSquare { c: _ } => { print!("#") }
                    StartSquare { c: _, orientation } => {
                        let o = orientation.clone().pop().unwrap();
                        match o.0 {
                            HORIZONTAL => {
                                if o.1 == Direction::DIRECT {
                                    print!("→")
                                } else {
                                    print!("←")
                                }
                            }
                            VERTICAL => {
                                if o.1 == Direction::DIRECT {
                                    print!("↓")
                                } else {
                                    print!("↑")
                                }
                            }
                        }
                    }
                }

            } else {
                print!("_")
            }
        }
        println!()
    }
}
fn create_map(crossword: &Crossword) -> HashMap<(u8,u8), Square> {
    let mut squares: HashMap<(u8,u8), Square> = HashMap::new();
    let words = crossword.get_words();
    words.iter().for_each(|w| {
        let i = w.get_i();
        let j = w.get_j();
        // insert or update the start square
        let square;
        if squares.contains_key(&(i, j)) {
            square = squares.get(&(i, j)).unwrap().add_orientation((w.get_orientation(), w.get_direction()))
        } else {
            square = StartSquare{c: w.get_word().chars().nth(0).unwrap(), orientation: vec![(w.get_orientation(), w.get_direction())]}
        }
        squares.insert((i, j), square);
        for x in 1..w.get_word().len() {
            if w.get_orientation() == HORIZONTAL {
                let new_i;
                if w.get_direction() == Direction::DIRECT {
                    new_i = i+(x as u8);
                } else {
                    new_i = i-(x as u8);
                }
                squares.insert((new_i, j), CharSquare{c: w.get_word().chars().nth(x).unwrap()});
            } else {
                let new_j;
                if w.get_direction() == Direction::DIRECT {
                    new_j = j+(x as u8);
                } else {
                    new_j = j-(x as u8);
                }
                squares.insert((i, new_j), CharSquare{c: w.get_word().chars().nth(x).unwrap()});
            }

        }
    });

    return squares
}

#[derive(Debug, Clone)]
enum Square {
    CharSquare{c: char},
    StartSquare{c: char, orientation: Vec<(Orientation, Direction)>}
}

impl Square {
    fn add_orientation(&self, new_orientation: (Orientation, Direction)) -> crate::Square {
        return match self {
            CharSquare { c } => StartSquare{c: *c, orientation: vec![new_orientation] },
            StartSquare { c, orientation } => {
                let mut adding_orientation = vec![];
                adding_orientation.append(orientation.clone().as_mut());
                adding_orientation.push(new_orientation);
                StartSquare {c: *c, orientation: adding_orientation}
            }
        }
    }
}