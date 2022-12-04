use std::{ops::RangeInclusive, str::FromStr};

use color_eyre::{eyre::ContextCompat, Report, Result};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let a: Assignments = input.parse()?;

    let count = a.count_containment();
    println!("There are {count} pairs fully containing the other");
    let count = a.count_overlaps();
    println!("There are {count} pairs overlapping the other");

    Ok(())
}

struct Assignment {
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

impl Assignment {
    fn contains_other(&self) -> bool {
        self.first.contains(self.second.start()) && self.first.contains(self.second.end())
            || self.second.contains(self.first.start()) && self.second.contains(self.first.end())
    }

    fn overlaps_other(&self) -> bool {
        self.first.contains(self.second.start())
            || self.first.contains(self.second.end())
            || self.second.contains(self.first.start())
            || self.second.contains(self.first.end())
    }
}

static PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<first_start>\d+)-(?P<first_end>\d+),(?P<second_start>\d+)-(?P<second_end>\d+)")
        .unwrap()
});

impl FromStr for Assignment {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = PATTERN
            .captures(s)
            .wrap_err_with(|| format!("Invalid line {s:?}"))?;
        let first_start = c.name("first_start").unwrap().as_str().parse()?;
        let first_end = c.name("first_end").unwrap().as_str().parse()?;
        let second_start = c.name("second_start").unwrap().as_str().parse()?;
        let second_end = c.name("second_end").unwrap().as_str().parse()?;
        Ok(Assignment {
            first: first_start..=first_end,
            second: second_start..=second_end,
        })
    }
}

struct Assignments(Vec<Assignment>);

impl Assignments {
    fn count_containment(&self) -> usize {
        self.0.iter().filter(|&a| a.contains_other()).count()
    }

    fn count_overlaps(&self) -> usize {
        self.0.iter().filter(|&a| a.overlaps_other()).count()
    }
}

impl FromStr for Assignments {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim().lines().map(|l| l.parse()).collect::<Result<_>>()?,
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
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "}
    }

    #[rstest]
    fn test_assignments(input: &str) {
        let a: Assignments = input.parse().unwrap();

        assert_eq!(a.count_containment(), 2);
        assert_eq!(a.count_overlaps(), 4);
    }
}
