use std::cmp::Ordering;
use crate::crossword::word::Direction;
use crate::crossword::word::Orientation::{HORIZONTAL, VERTICAL};

pub(crate) mod word;

pub struct Crossword {
    words: Vec<word::Word>,
}

impl Crossword {
    pub fn get_words(&self) -> Vec<word::Word> {
        return self.words.clone()
    }

    pub fn get_size(&self) -> (u8, u8) {
        let horizontal = self.words.iter().filter(|x| x.get_orientation() == HORIZONTAL)
            .map(|x| {
                let mut max = x.get_i();
                if x.get_direction() == Direction::DIRECT {
                    max = max + (x.get_word().len() as u8)
                }
                return max
            })
            .max().unwrap();
        let vertical = self.words.iter().filter(|x| x.get_orientation() == VERTICAL)
            .map(|x| {
                let mut max = x.get_j();
                if x.get_direction() == Direction::DIRECT {
                    max = max + (x.get_word().len() as u8)
                }
                return max
            })
            .max().unwrap();


        return (horizontal, vertical)

    }
}

pub fn build_crossword() -> Crossword {
    // hardcoded words for now
    let sugar = word::build_word(String::from("sugar"), String::from("You need two of these to craft a Moonshine"), 0, 3, HORIZONTAL, Direction::DIRECT);
    let scav = word::build_word(String::from("scav"), String::from("Enemies all over the map"), 0, 7, HORIZONTAL, Direction::DIRECT);
    let ushanka = word::build_word(String::from("ushanka"), String::from("Russian hat"), 2, 5, HORIZONTAL, Direction::DIRECT);
    let gluhar = word::build_word(String::from("gluhar"), String::from("Reserve boss"), 2, 3, VERTICAL, Direction::DIRECT);
    let moonshine = word::build_word(String::from("moonshine"), String::from("Alcoholic drink that cost a lot of roubles"), 6, 8, VERTICAL, Direction::REVERSE);

    let mut words = Vec::new();

    words.push(sugar);
    words.push(scav);
    words.push(ushanka);

    words.push(gluhar);
    words.push(moonshine);

    let mut v: Vec<word::Word> = words.clone();
    v.sort_by(|a,b| {
        if a.get_orientation() == b.get_orientation() {
            if a.get_orientation() == HORIZONTAL {
                return a.get_j().cmp(&b.get_j())
            } else {
                return a.get_i().cmp(&b.get_i())
            }

        } else {
            return if a.get_orientation() == HORIZONTAL {Ordering::Less} else {Ordering::Greater}
        }
    });


    return Crossword {
        words: v,
    }
}