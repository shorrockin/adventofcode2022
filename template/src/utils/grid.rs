use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate(pub i32, pub i32);

impl core::ops::Add<Direction> for Coordinate {
    type Output = Coordinate;
    fn add(self, direction: Direction) -> Coordinate {
        Coordinate(self.0 + direction.0, self.1 + direction.1)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Direction(pub i32, pub i32);

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
    pub points: HashMap<Coordinate, P>,
    pub max_width: i32,
    pub max_height: i32,
}

pub trait Point {
    fn symbol(&self) -> String;
    fn coord(&self) -> Coordinate;
    fn distance(&self, other: &Self) -> i32 {
        let other_coord = other.coord();
        let self_coord = self.coord();
        (other_coord.0 - self_coord.0).abs() + (other_coord.1 - self_coord.1).abs()
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
                max_width = max_width.max(coord.0);
                max_height = max_height.max(coord.1);
                points.insert(coord, creator(coord, symbol));
            })
        });
        Grid {
            points,
            max_width,
            max_height,
        }
    }

    pub fn at(&self, position: Coordinate) -> Option<&P> {
        self.points.get(&position)
    }

    pub fn at_relative(&self, relative_to: &P, direction: Direction) -> Option<&P> {
        let relative_pos = relative_to.coord();
        self.at(relative_pos + direction)
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

    // renders the grid in its original form, renderer provided if you want to
    // add coloring, or change from the initial symbol used to populated
    pub fn render<R>(&self, render: R) -> String
    where
        R: Fn(&P) -> String,
    {
        let string_list: Vec<String> = (0..=self.max_height)
            .map(|y| {
                (0..=self.max_width)
                    .map(|x| match self.at(Coordinate(x, y)) {
                        Some(point) => render(point),
                        None => " ".to_string(),
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
    symbol: char,
}

impl Point for BasicPoint {
    fn coord(&self) -> Coordinate {
        self.coord
    }

    fn symbol(&self) -> String {
        String::from(self.symbol)
    }
}

impl BasicPoint {
    pub fn new(coord: Coordinate, symbol: char) -> BasicPoint {
        BasicPoint { coord, symbol }
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
            grid.at(Coordinate(0, 0))
        );
        assert_eq!(
            Some(&BasicPoint::new(Coordinate(6, 2), 'U')),
            grid.at(Coordinate(6, 2))
        );
        assert_eq!(
            Some(&BasicPoint::new(Coordinate(3, 1), 'K')),
            grid.at(Coordinate(3, 1))
        );
    }

    #[test]
    fn test_grid_movement() {
        let grid = Grid::from(GRID_STR, BasicPoint::new);
        assert_eq!(None, grid.north(grid.at(Coordinate(0, 0)).unwrap()));
        assert_eq!(None, grid.west(grid.at(Coordinate(0, 0)).unwrap()));
        assert_eq!(
            'H',
            grid.south(grid.at(Coordinate(0, 0)).unwrap())
                .unwrap()
                .symbol
        );
        assert_eq!(
            'B',
            grid.east(grid.at(Coordinate(0, 0)).unwrap())
                .unwrap()
                .symbol
        );
        assert_eq!(
            'A',
            grid.north(grid.at(Coordinate(0, 1)).unwrap())
                .unwrap()
                .symbol
        );
        assert_eq!(
            'A',
            grid.west(grid.at(Coordinate(1, 0)).unwrap())
                .unwrap()
                .symbol
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
