use core::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Movement {
    Forward(usize),
    TurnRight,
    TurnLeft,
}
impl Movement {
    pub fn from(line: &str) -> Vec<Movement> {
        let mut out = vec![];
        let mut current_digit = "".to_string();
        for character in line.chars() {
            match character {
                'R' => {
                    out.push(Movement::Forward(current_digit.parse().unwrap()));
                    current_digit = "".to_string();
                    out.push(Movement::TurnRight);
                }
                'L' => {
                    out.push(Movement::Forward(current_digit.parse().unwrap()));
                    current_digit = "".to_string();
                    out.push(Movement::TurnLeft);
                }
                _ => {
                    current_digit.push(character);
                }
            }
        }

        out.push(Movement::Forward(current_digit.parse().unwrap()));
        out
    }
}
impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Movement::Forward(amount) => write!(f, "Forward({amount})"),
            Movement::TurnRight => write!(f, "TurnRight"),
            Movement::TurnLeft => write!(f, "TurnLeft"),
        }
    }
}
