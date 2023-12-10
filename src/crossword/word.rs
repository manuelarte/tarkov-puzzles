#[derive(Debug, Clone)]
pub(crate) struct Word {
    word: String,
    description: String,
    i: u8,
    j: u8,
    orientation: Orientation,
    direction: Direction,
}

impl Word {
    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_i(&self) -> u8 {
        self.i
    }

    pub fn get_j(&self) -> u8 {
        self.j
    }
}

pub fn build_word(word: String, description: String, i: u8, j: u8, orientation: Orientation, direction: Direction) -> Word {
    Word{
        word, description, i, j, orientation, direction
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    HORIZONTAL,
    VERTICAL,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    DIRECT,
    REVERSE,
}