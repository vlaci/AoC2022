use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use std::{collections::VecDeque, fmt::Display, str::FromStr};

use color_eyre::{Report, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let monkeys: Monkeys = input.parse().unwrap();

    let mut part1 = monkeys.clone();
    for r in 1..=20 {
        part1.round();
        println!(
            "After round {r}, the monkeys are holding items with these worry levels:\n{monkeys}"
        );
    }

    println!("The monkey business score is {}", part1.business());

    let mut part2 = monkeys;
    part2.bored = false;
    for _ in 1..=10_000 {
        part2.round();
    }

    println!("The monkey business score is {}", part2.business());

    Ok(())
}
#[derive(Clone)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspects: u64,
}

#[derive(Clone)]
enum Operation {
    Add(Arg),
    Mul(Arg),
}

#[derive(Clone)]
enum Arg {
    Old,
    Const(u64),
}

#[derive(Clone)]
struct Test {
    divisible_by: u64,
    if_true: u64,
    if_false: u64,
}

impl Operation {
    fn execute(&self, level: u64) -> u64 {
        match self {
            Operation::Add(Arg::Old) => level + level,
            Operation::Add(Arg::Const(c)) => level + c,
            Operation::Mul(Arg::Old) => level * level,
            Operation::Mul(Arg::Const(c)) => level * c,
        }
    }
}
impl Test {
    fn test(&self, level: u64) -> usize {
        if level % self.divisible_by == 0 {
            self.if_true as usize
        } else {
            self.if_false as usize
        }
    }
}

impl Monkey {
    fn parse(s: &str) -> IResult<&str, Self> {
        fn number(s: &str) -> IResult<&str, u64> {
            nom::character::complete::u64(s)
        }

        fn monkey_number(s: &str) -> IResult<&str, u64> {
            delimited(tag("Monkey "), number, terminated(tag(":"), line_ending))(s)
        }

        fn items(s: &str) -> IResult<&str, Vec<u64>> {
            delimited(
                tag("  Starting items: "),
                separated_list1(tag(", "), number),
                line_ending,
            )(s)
        }

        fn operation(s: &str) -> IResult<&str, Operation> {
            fn arg(s: &str) -> IResult<&str, Arg> {
                alt((map(tag("old"), |_| Arg::Old), map(number, Arg::Const)))(s)
            }
            delimited(
                tag("  Operation: new = old"),
                alt((
                    map(preceded(tag(" + "), arg), Operation::Add),
                    map(preceded(tag(" * "), arg), Operation::Mul),
                )),
                line_ending,
            )(s)
        }

        fn test(s: &str) -> IResult<&str, Test> {
            fn cond(c: bool) -> impl FnMut(&str) -> IResult<&str, u64> {
                move |s: &str| {
                    delimited(
                        tag(format!("    If {c}: throw to monkey ").as_str()),
                        number,
                        line_ending,
                    )(s)
                }
            }
            map(
                tuple((
                    delimited(tag("  Test: divisible by "), number, line_ending),
                    cond(true),
                    cond(false),
                )),
                |(divisible_by, if_true, if_false)| Test {
                    divisible_by,
                    if_true,
                    if_false,
                },
            )(s)
        }

        map(
            tuple((monkey_number, items, operation, test)),
            |(id, items, operation, test)| Monkey {
                id,
                items: items.into(),
                operation,
                test,
                inspects: 0,
            },
        )(s)
    }
}

impl FromStr for Monkey {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s).map_err(|err| err.to_owned())?.1)
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey {}: {}",
            self.id,
            self.items
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    bored: bool,
    lcm: u64,
}

impl FromStr for Monkeys {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rv: Vec<Monkey> = all_consuming(separated_list1(line_ending, Monkey::parse))(s)
            .map_err(|err| err.to_owned())?
            .1
            .into_iter()
            .collect();
        rv.sort_by_key(|m| m.id);
        let lcm = rv
            .iter()
            .map(|m| m.test.divisible_by)
            .reduce(num::integer::lcm)
            .unwrap();

        Ok(Self {
            monkeys: rv,
            lcm,
            bored: true,
        })
    }
}

impl Display for Monkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.monkeys.iter().try_for_each(|m| writeln!(f, "{}", m))
    }
}

impl Monkeys {
    fn round(&mut self) {
        for m in 0..self.monkeys.len() {
            while let Some(level) = self.monkeys[m].items.pop_front() {
                self.monkeys[m].inspects += 1;
                let mut level = self.monkeys[m].operation.execute(level) % self.lcm;
                if self.bored {
                    level /= 3;
                }
                let dest = self.monkeys[m].test.test(level);
                self.monkeys[dest].items.push_back(level);
            }
        }
    }

    fn business(&self) -> u64 {
        let mut counts = self.monkeys.iter().map(|m| m.inspects).collect::<Vec<_>>();
        counts.sort();
        counts.pop().unwrap() * counts.pop().unwrap()
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
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
        "}
    }

    #[fixture]
    #[once]
    fn monkeys(input: &str) -> Monkeys {
        input.parse().unwrap()
    }

    #[rstest]
    fn test_parsing(input: &str) {
        let _: Monkey = input.parse().unwrap();

        let monkeys: Monkeys = input.parse().unwrap();

        assert_eq!(
            monkeys.monkeys.iter().map(|m| m.id).collect::<Vec<_>>(),
            Vec::from_iter(0..=3)
        );
    }

    #[rstest]
    fn test_bored(monkeys: &Monkeys) {
        let mut monkeys = monkeys.clone();
        for r in 1..=20 {
            monkeys.round();
            println!("After round {r}, the monkeys are holding items with these worry levels:\n{monkeys}");
        }

        assert_eq!(monkeys.business(), 10605);
    }
    #[rstest]
    fn test_not_bored(monkeys: &Monkeys) {
        let mut monkeys = monkeys.clone();
        monkeys.bored = false;
        for _r in 1..=10_000 {
            monkeys.round();
        }
        assert_eq!(monkeys.business(), 2713310158);
    }
}
