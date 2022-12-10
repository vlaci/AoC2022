use std::{cmp, collections::HashSet, fmt::Display, ops, str::FromStr};

use color_eyre::{
    eyre::{eyre, ContextCompat},
    Report, Result,
};

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let moves = parse_moves(&input).unwrap();
    let mut r = Rope::new();
    moves.iter().for_each(|m| {
        r.step(m);
    });

    println!("The tail touched {} positions", r.touched());
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ops::Mul<i32> for Direction {
    type Output = Direction;

    fn mul(self, rhs: i32) -> Self::Output {
        match rhs.signum() {
            1 => self,
            -1 => match self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            },
            _ => unreachable!(),
        }
    }
}

impl FromStr for Direction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(eyre!(format!("Invalid input '{s}'"))),
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord(i32, i32);

impl ops::AddAssign<Direction> for Coord {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: usize,
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, s) = s
            .trim()
            .split_once(' ')
            .wrap_err_with(|| "Invalid move '{s}'")?;

        Ok(Self {
            direction: d.parse()?,
            steps: s.parse()?,
        })
    }
}

#[derive(Default)]
struct Rope {
    head: Coord,
    tail: Coord,
    touched: HashSet<Coord>,
}

impl Rope {
    fn new() -> Self {
        let mut r: Self = Default::default();
        r.touched.insert(r.tail);
        r
    }

    fn step(&mut self, m: &Move) {
        for _ in 0..m.steps {
            self.head += m.direction;

            let dx = self.head.0 - self.tail.0;
            let dy = self.head.1 - self.tail.1;

            if dx.abs() > 1 {
                self.tail += Direction::Right * dx;
                if dy.abs() == 1 {
                    self.tail += Direction::Up * dy;
                }
                self.touched.insert(self.tail);
            }

            if dy.abs() > 1 {
                self.tail += Direction::Up * dy;
                if dx.abs() == 1 {
                    self.tail += Direction::Right * dx;
                }
                self.touched.insert(self.tail);
            }
        }
    }

    fn touched(&self) -> usize {
        self.touched.len()
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = cmp::max(
            self.touched.iter().map(|c| c.0).max().unwrap_or(0),
            self.head.0,
        );
        let height = cmp::max(
            self.touched.iter().map(|c| c.1).max().unwrap_or(0),
            self.head.1,
        );
        f.write_fmt(format_args!(
            "head: {}, {}, tail {}, {}\n",
            self.head.0, self.head.1, self.tail.0, self.tail.1,
        ))?;
        for y in (0..=height).rev() {
            for x in 0..=width {
                let coord = Coord(x, y);
                if coord == self.head {
                    f.write_str("H")?;
                } else if coord == self.tail {
                    f.write_str("T")?;
                } else if self.touched.contains(&coord) {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn parse_moves(input: &str) -> Result<Vec<Move>> {
    input.lines().map(str::parse).collect::<Result<_>>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use pretty_assertions::assert_eq;
    use rstest::*;

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "}
    }

    #[rstest]
    fn test_parsing(input: &str) {
        let moves = parse_moves(input).unwrap();
        let mut r = Rope::new();
        moves.iter().for_each(|m| {
            r.step(m);
        });

        assert_eq!(r.touched(), 13);
    }
}
