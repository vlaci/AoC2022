use lending_iterator::prelude::*;
use std::{collections::HashSet, ops, str::FromStr};

use color_eyre::{
    eyre::{eyre, ContextCompat},
    Report, Result,
};

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let moves = parse_moves(&input).unwrap();
    let mut r: Rope<2> = Rope::new();
    moves.iter().for_each(|m| {
        r.step(m);
    });

    println!("The tail touched {} positions", r.touched());
    let mut r: Rope<10> = Rope::new();
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

struct Rope<const N: usize> {
    coords: [Coord; N],
    touched: HashSet<Coord>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Rope<N> {
        let mut r = Rope {
            coords: [Default::default(); N],
            touched: Default::default(),
        };
        r.touched.insert(Default::default());
        r
    }

    fn step(&mut self, m: &Move) {
        for _ in 0..m.steps {
            self.coords[0] += m.direction;
            let mut windows = self.coords.windows_mut::<2>();
            let mut i = 1;
            while let Some(&mut [head, ref mut tail]) = windows.next() {
                i += 1;

                let dx = head.0 - tail.0;
                let dy = head.1 - tail.1;

                if dx.abs() > 1 {
                    *tail += Direction::Right * dx;
                    if dy.abs() == 1 {
                        *tail += Direction::Up * dy;
                    }
                }

                if dy.abs() > 1 {
                    *tail += Direction::Up * dy;
                    if dx.abs() == 1 {
                        *tail += Direction::Right * dx;
                    }
                }
                if i == N {
                    self.touched.insert(*tail);
                }
            }
        }
    }

    fn touched(&self) -> usize {
        self.touched.len()
    }

    #[allow(dead_code)]
    fn print(&self, a: (i32, i32), b: (i32, i32)) {
        for y in (a.1..=b.1).rev() {
            for x in a.0..=b.0 {
                let coord = Coord(x, y);
                if let Some(i) = self.coords.iter().position(|&c| c == coord) {
                    eprint!("{i}");
                } else if self.touched.contains(&coord) {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        eprintln!();
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
        let mut r: Rope<2> = Rope::new();
        moves.iter().for_each(|m| {
            r.step(m);
        });

        assert_eq!(r.touched(), 13);
        let mut r: Rope<10> = Rope::new();
        moves.iter().for_each(|m| {
            r.step(m);
        });
        assert_eq!(r.touched(), 1);
    }

    #[rstest]
    fn test_long() {
        let input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        let mut r: Rope<10> = Rope::new();

        let moves = parse_moves(input).unwrap();
        moves.iter().for_each(|m| {
            r.step(m);
        });

        assert_eq!(r.touched(), 36);
    }
}
