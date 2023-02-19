// defines the parse instructions for input, input is always a 6 sided cube with
// fixed dimensions, with different quadrants having different locations in our
// input.
pub struct Cube {
    pub size: i32,
    pub sides: Vec<Side>,
}
impl Cube {
    pub fn new(size: i32, sides: Vec<Side>) -> Cube {
        Cube { size, sides }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub enum Face {
    Top,
    Front,
    Left,
    Right,
    Bottom,
    Back,
}
impl Face {
    // applies a rotation based on the x, y, z direction. only one should be
    // non-null, can be negative.
    pub fn rotate(&self, x: i32, y: i32, z: i32) -> Face {
        match self {
            Face::Top | Face::Bottom => {
                Face::next(x, Face::Right, Face::Left, z, Face::Front, Face::Back)
            }
            Face::Left | Face::Right => {
                Face::next(z, Face::Front, Face::Back, y, Face::Bottom, Face::Top)
            }
            Face::Front | Face::Back => {
                Face::next(x, Face::Right, Face::Left, y, Face::Bottom, Face::Top)
            }
        }
    }

    fn next(
        first: i32,
        pos_first: Face,
        neg_first: Face,
        second: i32,
        pos_second: Face,
        neg_second: Face,
    ) -> Face {
        match (first, second) {
            (1, 0) => pos_first,
            (-1, 0) => neg_first,
            (0, 1) => pos_second,
            (0, -1) => neg_second,
            _ => panic!("invalid arguments passed into next facing {first}/{second}"),
        }
    }
}

// parse instruction for a specific side, back left should be 0, 0, 0 to
// avoid negative values. start is the position to start parsing in the
// input for this side, and translator converts the x/y coords into x/y/z coords.
pub struct Side {
    pub face: Face,
    pub start: (i32, i32),
    pub translator: fn(i32, i32) -> (i32, i32, i32),
}
impl Side {
    pub fn new(face: Face, x: i32, y: i32, translator: fn(i32, i32) -> (i32, i32, i32)) -> Side {
        Side {
            face,
            start: (x, y),
            translator,
        }
    }
}
