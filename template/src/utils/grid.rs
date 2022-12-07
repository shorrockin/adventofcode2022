use std::collections::HashMap;

pub struct RelativeDirection {
    x: i32,
    y: i32,
}

pub mod directions {
    use super::*;
    pub static NORTH: RelativeDirection = RelativeDirection { x: 0, y: -1 };
    pub static SOUTH: RelativeDirection = RelativeDirection { x: 0, y: 1 };
    pub static EAST: RelativeDirection = RelativeDirection { x: 1, y: 0 };
    pub static WEST: RelativeDirection = RelativeDirection { x: -1, y: 0 };
}

#[derive(Debug)]
pub struct Grid {
    points: HashMap<(i32, i32), Point>,
    max_width: i32,
    max_height: i32,
}

impl Grid {
    pub fn from_string(input: &str) -> Grid {
        let mut max_width = 0;
        let mut max_height = 0;
        let mut points = HashMap::new();
        input.lines().enumerate().for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(char_index, symbol)| {
                let x = char_index.try_into().unwrap();
                let y = line_index.try_into().unwrap();
                max_width = if max_width > x { max_width } else { x };
                max_height = if max_height > y { max_height } else { y };
                points.insert((x, y), Point { x, y, symbol });
            })
        });
        Grid {
            points,
            max_width,
            max_height,
        }
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&Point> {
        self.points.get(&(x, y))
    }

    pub fn at_relative(
        &self,
        relative_to: &Point,
        relative_direction: &RelativeDirection,
    ) -> Option<&Point> {
        self.at(
            relative_to.x + relative_direction.x,
            relative_to.y + relative_direction.y,
        )
    }

    pub fn north(&self, source: &Point) -> Option<&Point> {
        self.at_relative(source, &directions::NORTH)
    }

    pub fn south(&self, source: &Point) -> Option<&Point> {
        self.at_relative(source, &directions::SOUTH)
    }

    pub fn east(&self, source: &Point) -> Option<&Point> {
        self.at_relative(source, &directions::EAST)
    }

    pub fn west(&self, source: &Point) -> Option<&Point> {
        self.at_relative(source, &directions::WEST)
    }

    pub fn pretty_print(&self) -> String {
        let string_list: Vec<String> = (0..=self.max_height)
            .map(|y| {
                (0..=self.max_width)
                    .map(|x| match self.at(x, y) {
                        Some(point) => point.symbol,
                        None => ' ',
                    })
                    .collect::<String>()
            })
            .collect();
        string_list.join("\n")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
    symbol: char,
}

impl Point {
    pub fn distance(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
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

    fn point(x: i32, y: i32, symbol: char) -> Point {
        Point { x, y, symbol }
    }

    #[test]
    fn test_grid_from_string() {
        let grid = Grid::from_string(GRID_STR);
        assert_eq!(Some(&point(0, 0, 'A')), grid.at(0, 0));
        assert_eq!(Some(&point(6, 2, 'U')), grid.at(6, 2));
        assert_eq!(Some(&point(3, 1, 'K')), grid.at(3, 1));
    }

    #[test]
    fn test_grid_movement() {
        let grid = Grid::from_string(GRID_STR);
        assert_eq!(None, grid.north(grid.at(0, 0).unwrap()));
        assert_eq!(None, grid.west(grid.at(0, 0).unwrap()));
        assert_eq!('H', grid.south(grid.at(0, 0).unwrap()).unwrap().symbol);
        assert_eq!('B', grid.east(grid.at(0, 0).unwrap()).unwrap().symbol);
        assert_eq!('A', grid.north(grid.at(0, 1).unwrap()).unwrap().symbol);
        assert_eq!('A', grid.west(grid.at(1, 0).unwrap()).unwrap().symbol);
        assert_eq!(GRID_STR.trim(), grid.pretty_print());
    }

    #[test]
    fn test_point_distance() {
        assert_eq!(3, point(1, 1, 'A').distance(&point(2, 3, 'B')));
    }
}
