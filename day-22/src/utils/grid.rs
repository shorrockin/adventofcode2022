use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate(pub i32, pub i32);

impl core::ops::Add<Direction> for Coordinate {
    type Output = Coordinate;
    fn add(self, direction: Direction) -> Coordinate {
        Coordinate(self.0 + direction.0, self.1 + direction.1)
    }
}

impl Coordinate {
    pub fn from(value: &str) -> Coordinate {
        let (x, y) = value.split_once(',').unwrap();
        Coordinate(x.parse().unwrap(), y.parse().unwrap())
    }

    pub fn to(&self, end: Coordinate) -> CoordinateIterator {
        CoordinateIterator::new(*self, end)
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Coordinate({},{})", self.0, self.1))
    }
}

pub struct CoordinateIterator {
    next: Coordinate,
    step: Direction,
    end: Coordinate,
}

impl std::iter::Iterator for CoordinateIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.eq(&self.end) {
            return None;
        }

        let out = Some(self.next);
        self.next = self.next + self.step;

        out
    }
}

impl CoordinateIterator {
    fn new(from: Coordinate, to: Coordinate) -> CoordinateIterator {
        let step = Direction((to.0 - from.0).signum(), (to.1 - from.1).signum());
        CoordinateIterator {
            next: from,
            step,
            end: (to + step),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Direction(pub i32, pub i32);
impl Direction {
    pub fn turn_right(&self) -> Direction {
        if *self == directions::NORTH {
            directions::EAST
        } else if *self == directions::EAST {
            directions::SOUTH
        } else if *self == directions::SOUTH {
            directions::WEST
        } else if *self == directions::WEST {
            directions::NORTH
        } else {
            panic!("can't rotate a inter-cardinal direction")
        }
    }

    pub fn turn_left(&self) -> Direction {
        if *self == directions::NORTH {
            directions::WEST
        } else if *self == directions::EAST {
            directions::NORTH
        } else if *self == directions::SOUTH {
            directions::EAST
        } else if *self == directions::WEST {
            directions::SOUTH
        } else {
            panic!("can't rotate a inter-cardinal direction")
        }
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Direction({},{})", self.0, self.1))
    }
}

pub mod directions {
    use super::*;
    pub static NORTH: Direction = Direction(0, -1);
    pub static NORTH_WEST: Direction = Direction(-1, -1);
    pub static NORTH_EAST: Direction = Direction(1, -1);
    pub static SOUTH: Direction = Direction(0, 1);
    pub static SOUTH_WEST: Direction = Direction(-1, 1);
    pub static SOUTH_EAST: Direction = Direction(1, 1);
    pub static EAST: Direction = Direction(1, 0);
    pub static WEST: Direction = Direction(-1, 0);
}

#[derive(Debug)]
pub struct Grid<P: Point> {
    points: HashMap<Coordinate, P>,
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,
}

pub trait Point {
    fn symbol(&self) -> &str;
    fn coord(&self) -> Coordinate;
    fn distance(&self, other: &Self) -> i32 {
        let other_coord = other.coord();
        let self_coord = self.coord();
        (other_coord.0 - self_coord.0).abs() + (other_coord.1 - self_coord.1).abs()
    }
    fn ignore(&self) -> bool {
        false
    }
}

impl<P: Point> Grid<P> {
    pub fn from<F>(input: &str, creator: F) -> Grid<P>
    where
        F: Fn(Coordinate, char) -> P,
    {
        let mut max_width = 0;
        let mut max_height = 0;
        let mut points: HashMap<Coordinate, P> = HashMap::new();
        input.lines().enumerate().for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(char_index, symbol)| {
                let coord = Coordinate(char_index as i32, line_index as i32);
                let point = creator(coord, symbol);
                if !point.ignore() {
                    max_width = max_width.max(coord.0);
                    max_height = max_height.max(coord.1);
                    points.insert(coord, creator(coord, symbol));
                }
            })
        });
        Grid {
            points,
            min_width: 0,
            min_height: 0,
            max_width,
            max_height,
        }
    }

    pub fn new() -> Grid<P> {
        Grid {
            points: HashMap::new(),
            min_width: i32::MAX,
            min_height: i32::MAX,
            max_width: i32::MIN,
            max_height: i32::MIN,
        }
    }

    pub fn insert(&mut self, point: P) {
        let at = point.coord();
        self.min_width = self.min_width.min(at.0);
        self.min_height = self.min_height.min(at.1);
        self.max_width = self.max_width.max(at.0);
        self.max_height = self.max_height.max(at.1);
        self.points.insert(at, point);
    }

    pub fn at(&self, position: &Coordinate) -> Option<&P> {
        self.points.get(position)
    }

    pub fn at_relative(&self, relative_to: &P, direction: Direction) -> Option<&P> {
        let relative_pos = relative_to.coord();
        self.at(&(relative_pos + direction))
    }

    pub fn north(&self, source: &P) -> Option<&P> {
        self.at_relative(source, directions::NORTH)
    }

    pub fn south(&self, source: &P) -> Option<&P> {
        self.at_relative(source, directions::SOUTH)
    }

    pub fn east(&self, source: &P) -> Option<&P> {
        self.at_relative(source, directions::EAST)
    }

    pub fn west(&self, source: &P) -> Option<&P> {
        self.at_relative(source, directions::WEST)
    }

    pub fn out_of_bounds(&self, coordinate: Coordinate) -> bool {
        coordinate.0 > self.max_width
            || coordinate.0 < self.min_width
            || coordinate.1 > self.max_height
            || coordinate.1 < self.min_height
    }

    // scans from a starting point, in a direction, and returns the first point found
    pub fn scan(&self, starting: Coordinate, direction: Direction) -> Option<&P> {
        let mut current = starting;

        loop {
            if let Some(point) = self.at(&current) {
                return Some(point);
            }
            current = current + direction;
            if self.out_of_bounds(current) {
                return None;
            }
        }
    }

    // renders the grid in its original form, renderer provided if you want to
    // add coloring, or change from the initial symbol used to populated
    pub fn render<R>(&self, render: R) -> String
    where
        R: Fn(&P) -> &str,
    {
        let string_list: Vec<String> = (self.min_height..=self.max_height)
            .map(|y| {
                (self.min_width..=self.max_width)
                    .map(|x| match self.at(&Coordinate(x, y)) {
                        Some(point) => render(point),
                        None => " ",
                    })
                    .collect::<String>()
            })
            .collect();
        string_list.join("\n")
    }

    pub fn pretty_print(&self) -> String {
        self.render(|p| p.symbol())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BasicPoint {
    coord: Coordinate,
    character: char,
    symbol: String,
}

impl Point for BasicPoint {
    fn coord(&self) -> Coordinate {
        self.coord
    }

    fn symbol(&self) -> &str {
        self.symbol.as_str()
    }
}

impl BasicPoint {
    pub fn new(coord: Coordinate, character: char) -> BasicPoint {
        BasicPoint {
            coord,
            character,
            symbol: character.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static GRID_STR: &str = "ABCDEFG\nHIJKLMN\nOPQRSTU";

    #[test]
    fn test_grid_from_string() {
        let grid = Grid::from(GRID_STR, BasicPoint::new);
        assert_eq!(
            Some(&BasicPoint::new(Coordinate(0, 0), 'A')),
            grid.at(&Coordinate(0, 0))
        );
        assert_eq!(
            Some(&BasicPoint::new(Coordinate(6, 2), 'U')),
            grid.at(&Coordinate(6, 2))
        );
        assert_eq!(
            Some(&BasicPoint::new(Coordinate(3, 1), 'K')),
            grid.at(&Coordinate(3, 1))
        );
    }

    #[test]
    fn test_grid_movement() {
        let grid = Grid::from(GRID_STR, BasicPoint::new);
        assert_eq!(None, grid.north(grid.at(&Coordinate(0, 0)).unwrap()));
        assert_eq!(None, grid.west(grid.at(&Coordinate(0, 0)).unwrap()));
        assert_eq!(
            'H',
            grid.south(grid.at(&Coordinate(0, 0)).unwrap())
                .unwrap()
                .character
        );
        assert_eq!(
            'B',
            grid.east(grid.at(&Coordinate(0, 0)).unwrap())
                .unwrap()
                .character
        );
        assert_eq!(
            'A',
            grid.north(grid.at(&Coordinate(0, 1)).unwrap())
                .unwrap()
                .character
        );
        assert_eq!(
            'A',
            grid.west(grid.at(&Coordinate(1, 0)).unwrap())
                .unwrap()
                .character
        );
    }

    #[test]
    fn test_grid_pretty_print() {
        let grid = Grid::from(GRID_STR, BasicPoint::new);
        assert_eq!(GRID_STR.trim(), grid.pretty_print());
    }

    #[test]
    fn test_point_distance() {
        assert_eq!(
            3,
            BasicPoint::new(Coordinate(1, 1), 'A')
                .distance(&BasicPoint::new(Coordinate(2, 3), 'B'))
        );
    }
}
