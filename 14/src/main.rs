use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::all_consuming, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use std::{cmp, collections::HashSet, str::FromStr};

use color_eyre::{Report, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let map: Map = input.parse()?;

    let sand = map.pour_sand();

    println!("The amount of sand is {sand}");
    let sand = map.pour_sand2();
    println!("The amount of sand is {sand}");
    Ok(())
}

struct Map {
    cells: HashSet<(u32, u32)>,
    bottom: u32,
}

impl Map {
    fn new(rocks: impl Iterator<Item = (u32, u32)>) -> Self {
        let cells: HashSet<_> = rocks.collect();
        let bottom = *cells.iter().map(|(_x, y)| y).max().unwrap();
        Self { cells, bottom }
    }

    fn pour_sand(&self) -> usize {
        let mut amount = 0;
        let mut sand = (500, 0);
        let mut cells = self.cells.clone();

        while sand.1 <= self.bottom {
            if !cells.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;

                continue;
            } else if !cells.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;

                continue;
            } else if !cells.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;

                continue;
            } else if sand.1 == 0 {
                break;
            }

            cells.insert(sand);
            amount += 1;
            sand = (500, 0);
        }

        amount
    }

    fn pour_sand2(&self) -> usize {
        let bottom = self.bottom + 1;
        let mut amount = 0;
        let mut sand = (500, 0);
        let mut cells = self.cells.clone();

        loop {
            if sand.1 < bottom {
                if !cells.contains(&(sand.0, sand.1 + 1)) {
                    sand.1 += 1;

                    continue;
                } else if !cells.contains(&(sand.0 - 1, sand.1 + 1)) {
                    sand.0 -= 1;
                    sand.1 += 1;

                    continue;
                } else if !cells.contains(&(sand.0 + 1, sand.1 + 1)) {
                    sand.0 += 1;
                    sand.1 += 1;

                    continue;
                } else if sand.1 == 0 {
                    break;
                }
            }

            cells.insert(sand);
            amount += 1;
            sand = (500, 0);
        }

        amount + 1
    }
}

impl FromStr for Map {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn coord(s: &str) -> IResult<&str, (u32, u32)> {
            separated_pair(
                nom::character::complete::u32,
                tag(","),
                nom::character::complete::u32,
            )(s)
        }

        fn line(s: &str) -> IResult<&str, Vec<(u32, u32)>> {
            separated_list1(tag(" -> "), coord)(s)
        }

        Ok(Self::new(
            all_consuming(separated_list1(tag("\n"), line))(s.trim())
                .map_err(|e| e.to_owned())?
                .1
                .into_iter()
                .flat_map(|l| {
                    l.into_iter()
                        .tuple_windows()
                        .flat_map(|((sx, sy), (ex, ey))| {
                            (cmp::min(sx, ex)..=cmp::max(sx, ex))
                                .cartesian_product(cmp::min(sy, ey)..=cmp::max(sy, ey))
                        })
                }),
        ))
    }
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
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "}
    }

    #[rstest]
    fn test_sand(input: &str) {
        let map: Map = input.parse().unwrap();

        assert_eq!(map.pour_sand(), 24);
        assert_eq!(map.pour_sand2(), 93);
    }
}
