use color_eyre::{eyre::ContextCompat, Report, Result};
use itertools::{self, Itertools};
use once_cell::sync::Lazy;
use range_collections::{AbstractRangeSet, RangeSet2};
use regex::Regex;
use std::{ops::Range, str::FromStr};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let sensors = parse(&input)?;

    let row = 2_000_000;
    let count = count_non_beacon(&sensors, row);

    println!("{count} positions cannot contain a beacon at row {row}");

    let freq = calculate_tuning_frequency(&sensors, 4_000_000)
        .wrap_err("Couldn't determine tuning frequency")?;
    println!("The tuning frequency is {freq}");
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);
#[derive(Clone, Copy, Debug)]
struct Sensor {
    sensor: Pos,
    beacon: Pos,
    radius: i32,
}

static PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
        .unwrap()
});

impl FromStr for Sensor {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = PATTERN
            .captures(s)
            .wrap_err_with(|| format!("Invalid line {s}"))?;
        let s = Pos(cap[1].parse()?, cap[2].parse()?);
        let b = Pos(cap[3].parse()?, cap[4].parse()?);

        Ok(Sensor::new(s, b))
    }
}

fn manhattan(a: Pos, b: Pos) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn manhattan_slice(s: Sensor, y: i32) -> Range<i32> {
    let h = (s.sensor.1 - y).abs();
    let l = s.radius - h;

    s.sensor.0 - l..s.sensor.0 + l + 1
}

impl Sensor {
    fn new(s: Pos, b: Pos) -> Self {
        Self {
            sensor: s,
            beacon: b,
            radius: manhattan(s, b),
        }
    }
}

fn parse(s: &str) -> Result<Vec<Sensor>> {
    s.lines().map(|l| l.parse()).collect()
}

fn count_non_beacon(s: &[Sensor], r: i32) -> usize {
    s.iter()
        .filter(|s| s.sensor.1 + s.radius > r || s.sensor.1 - s.radius < r)
        .flat_map(|&s| manhattan_slice(s, r).filter(move |&p| Pos(p, r) != s.beacon))
        .unique()
        .count()
}

fn calculate_tuning_frequency(sensors: &[Sensor], limit: i32) -> Option<i64> {
    'y: for y in 0..=limit {
        let mut row = RangeSet2::from(0..limit + 1);
        for s in sensors {
            row.difference_with(&RangeSet2::from(manhattan_slice(*s, y)));
            if row.is_empty() {
                continue 'y;
            }
        }
        if let [x, _] = row.boundaries() {
            return Some(*x as i64 * 4_000_000 + y as i64);
        }
    }
    None
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
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "}
    }

    #[rstest]
    fn test_manhattan_slice() {
        let p = Sensor::new(Pos(1, 1), Pos(1, 3));
        assert_eq!(manhattan_slice(p, 1).collect::<Vec<_>>(), &[-1, 0, 1, 2, 3]);
        assert_eq!(manhattan_slice(p, 2).collect::<Vec<_>>(), &[0, 1, 2]);
        assert_eq!(manhattan_slice(p, 0).collect::<Vec<_>>(), &[0, 1, 2]);
    }

    #[rstest]
    fn test_beacon(input: &str) {
        let sensors = parse(input).unwrap();
        assert_eq!(sensors.len(), input.lines().count());

        assert_eq!(count_non_beacon(&sensors, 9), 25);
        assert_eq!(count_non_beacon(&sensors, 10), 26);
        assert_eq!(count_non_beacon(&sensors, 11), 28);

        assert_eq!(calculate_tuning_frequency(&sensors, 20), Some(56000011));
    }
}
