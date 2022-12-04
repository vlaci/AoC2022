use std::{collections::HashSet, str::FromStr};

use color_eyre::{eyre::eyre, Report, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let sacks = to_sacks(&input)?;
    let prio = sacks.iter().map(RuckSack::common).sum::<Result<u32>>()?;

    println!("The sum of priorities is {prio}");

    let groups = to_groups(&sacks).unwrap();

    let prio = groups
        .into_iter()
        .map(|p| p.into_iter().map(|p| *p).sum::<u32>())
        .sum::<u32>();

    println!("The sum of priorities is {prio}");

    Ok(())
}

struct RuckSack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

struct Priority(u32);

impl core::ops::Deref for Priority {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&char> for Priority {
    type Error = Report;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' => Ok(Self(*value as u32 - 0x60)),
            'A'..='Z' => Ok(Self(*value as u32 - 0x40 + 26)),
            _ => Err(eyre!(format!("Invalid item {value}"))),
        }
    }
}

impl RuckSack {
    fn common(&self) -> Result<u32> {
        let prios = self
            .first_compartment
            .intersection(&self.second_compartment)
            .map(Priority::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(prios.into_iter().map(|p| *p).sum())
    }

    fn common_2(&self) -> HashSet<char> {
        self.first_compartment
            .union(&self.second_compartment)
            .copied()
            .collect()
    }
}

fn to_sacks(input: &str) -> Result<Vec<RuckSack>> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<RuckSack>())
        .collect()
}

fn to_groups(sacks: &[RuckSack]) -> Result<Vec<Vec<Priority>>> {
    sacks
        .iter()
        .chunks(3)
        .into_iter()
        .map(|mut g| {
            let init = g.next().unwrap().common_2();
            g.map(|s| s.common_2())
                .fold(init, |acc, x| acc.intersection(&x).copied().collect())
                .iter()
                .map(Priority::try_from)
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}

impl FromStr for RuckSack {
    type Err = Report;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let count = value.len() / 2;
        let first_compartment = value.chars().take(count).collect::<HashSet<_>>();
        let second_compartment = value
            .chars()
            .skip(count)
            .take(count)
            .collect::<HashSet<_>>();
        Ok(Self {
            first_compartment,
            second_compartment,
        })
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
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "}
    }

    #[rstest]
    fn test_first(input: &str) {
        let sacks = to_sacks(input).unwrap();
        let prio = sacks
            .iter()
            .map(RuckSack::common)
            .sum::<Result<u32>>()
            .unwrap();

        assert_eq!(prio, 157);
    }
    #[rstest]
    fn test_second(input: &str) {
        let sacks = to_sacks(input).unwrap();
        let groups = to_groups(&sacks).unwrap();

        let sum = groups
            .into_iter()
            .map(|p| p.into_iter().map(|p| *p).sum::<u32>())
            .sum::<u32>();

        assert_eq!(sum, 70);
    }
}
