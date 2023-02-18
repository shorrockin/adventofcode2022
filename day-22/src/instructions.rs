// defines the parse instructions for input, input is always a 6 sided cube with
// fixed dimensions, with different quadrants having different locations in our
// input.
pub struct Cube {
    pub size: usize,
    pub sides: Vec<Side>,
}
impl Cube {
    pub fn new(size: usize, sides: Vec<Side>) -> Cube {
        Cube { size, sides }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub enum Facing {
    Top,
    Front,
    Left,
    Right,
    Bottom,
    Back,
}

// parse instruction for a specific side, back left should be 0, 0, 0 to
// avoid negative values. start is the position to start parsing in the
// input for this side, and translator converts the x/y coords into x/y/z coords.
pub struct Side {
    pub facing: Facing,
    pub start: (usize, usize),
    pub translator: fn(usize, usize) -> (usize, usize, usize),
}
impl Side {
    pub fn new(
        facing: Facing,
        x: usize,
        y: usize,
        translator: fn(usize, usize) -> (usize, usize, usize),
    ) -> Side {
        Side {
            facing,
            start: (x, y),
            translator,
        }
    }
}
