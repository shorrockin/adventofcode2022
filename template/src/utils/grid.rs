use std::collections::HashMap;

pub mod directions {
    pub static NORTH: (i32, i32) = (0, -1);
    pub static SOUTH: (i32, i32) = (0, 1);
    pub static EAST: (i32, i32) = (1, 0);
    pub static WEST: (i32, i32) = (-1, 0);
}

#[derive(Debug)]
pub struct Grid<P: Point> {
    points: HashMap<(i32, i32), P>,
    max_width: i32,
    max_height: i32,
}

pub trait Point {
    fn symbol(&self) -> char;
    fn coord(&self) -> (i32, i32);
    fn distance(&self, other: &Self) -> i32 {
        let other_coord = other.coord();
        let self_coord = self.coord();
        (other_coord.0 - self_coord.0).abs() + (other_coord.1 - self_coord.1).abs()
    }
}

impl<P: Point> Grid<P> {
    pub fn from<F>(input: &str, creator: F) -> Grid<P>
    where
        F: Fn(i32, i32, char) -> P,
    {
        let mut max_width = 0;
        let mut max_height = 0;
        let mut points: HashMap<(i32, i32), P> = HashMap::new();
        input.lines().enumerate().for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(char_index, symbol)| {
                let x = char_index.try_into().unwrap();
                let y = line_index.try_into().unwrap();
                max_width = if max_width > x { max_width } else { x };
                max_height = if max_height > y { max_height } else { y };
                points.insert((x, y), creator(x, y, symbol));
            })
        });
        Grid {
            points,
            max_width,
            max_height,
        }
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&P> {
        self.points.get(&(x, y))
    }

    pub fn at_relative(&self, relative_to: &P, direction: (i32, i32)) -> Option<&P> {
        let relative_pos = relative_to.coord();
        self.at(relative_pos.0 + direction.0, relative_pos.1 + direction.1)
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

    pub fn pretty_print(&self) -> String {
        let string_list: Vec<String> = (0..=self.max_height)
            .map(|y| {
                (0..=self.max_width)
                    .map(|x| match self.at(x, y) {
                        Some(point) => point.symbol(),
                        None => ' ',
                    })
                    .collect::<String>()
            })
            .collect();
        string_list.join("\n")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BasicPoint {
    x: i32,
    y: i32,
    symbol: char,
}

impl Point for BasicPoint {
    fn coord(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn symbol(&self) -> char {
        self.symbol
    }
}

impl BasicPoint {
    pub fn new(x: i32, y: i32, symbol: char) -> BasicPoint {
        BasicPoint { x, y, symbol }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static GRID_STR: &str = indoc! {"
        ABCDEFG
        HIJKLMN
        OPQRSTU
    "};

    #[test]
    fn test_grid_from_string() {
        let grid = Grid::from(GRID_STR, BasicPoint::new);
        assert_eq!(Some(&BasicPoint::new(0, 0, 'A')), grid.at(0, 0));
        assert_eq!(Some(&BasicPoint::new(6, 2, 'U')), grid.at(6, 2));
        assert_eq!(Some(&BasicPoint::new(3, 1, 'K')), grid.at(3, 1));
    }

    #[test]
    fn test_grid_movement() {
        let grid = Grid::from(GRID_STR, BasicPoint::new);
        assert_eq!(None, grid.north(grid.at(0, 0).unwrap()));
        assert_eq!(None, grid.west(grid.at(0, 0).unwrap()));
        assert_eq!('H', grid.south(grid.at(0, 0).unwrap()).unwrap().symbol);
        assert_eq!('B', grid.east(grid.at(0, 0).unwrap()).unwrap().symbol);
        assert_eq!('A', grid.north(grid.at(0, 1).unwrap()).unwrap().symbol);
        assert_eq!('A', grid.west(grid.at(1, 0).unwrap()).unwrap().symbol);
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
            BasicPoint::new(1, 1, 'A').distance(&BasicPoint::new(2, 3, 'B'))
        );
    }
}
