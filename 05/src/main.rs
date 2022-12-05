use std::str::FromStr;

use color_eyre::{eyre::ContextCompat, Report, Result};
use itertools::{iterate, Itertools};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let (stacks, moves) = parse(&input).unwrap();

    let mut part1 = stacks.clone();
    part1.execute(&moves)?;
    println!("The crates on the top are {}", part1.top());
    let mut part2 = stacks;
    part2.execute_batched(&moves)?;
    println!("The crates on the top are {}", part2.top());

    Ok(())
}

static CRATES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"((\[(?P<crate>\w)\] ?)|(?P<empty>    ))").unwrap());
static MOVES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap());

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<String>>);

impl Stacks {
    fn pop(&mut self, col: usize) -> Result<String> {
        self.0[col - 1]
            .pop()
            .wrap_err_with(|| format!("Column {col} is empty"))
    }

    fn push(&mut self, col: usize, item: String) {
        self.0[col - 1].push(item)
    }

    fn top(&self) -> String {
        self.0
            .iter()
            .filter_map(|s| s.last())
            .fold(String::new(), |acc, s| acc + s)
    }

    fn execute(&mut self, moves: &[Move]) -> Result<()> {
        for m in moves {
            for _ in 0..m.count {
                let item = self.pop(m.from)?;
                self.push(m.to, item);
            }
        }

        Ok(())
    }

    fn execute_batched(&mut self, moves: &[Move]) -> Result<()> {
        for m in moves {
            let stack = &mut self.0[m.from - 1];
            let mut items: Vec<_> = stack.drain(stack.len() - m.count..stack.len()).collect();
            let stack = &mut self.0[m.to - 1];
            stack.append(&mut items);
        }

        Ok(())
    }
}

impl FromStr for Stacks {
    type Err = Report;

    fn from_str(stacks: &str) -> Result<Self, Self::Err> {
        let mut stack_lines = stacks.lines().rev();
        let legend = stack_lines.next().wrap_err("invalid input")?;
        let columns = (legend.len() + 1) / 4;

        let mut stacks: Vec<Vec<String>> = iterate((), |_| ())
            .take(columns * (stacks.lines().count() - 1))
            .map(|_| Vec::new())
            .collect();

        for line in stack_lines {
            for (j, c) in CRATES.captures_iter(line).enumerate() {
                if let Some(c) = c.name("crate") {
                    stacks[j].push(c.as_str().into());
                }
            }
        }

        Ok(Self(stacks))
    }
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(m: &str) -> Result<Self, Self::Err> {
        let c = MOVES
            .captures(m)
            .wrap_err_with(|| format!("Invalid line {m}"))?;
        Ok(Move {
            count: c.name("count").unwrap().as_str().parse()?,
            from: c.name("from").unwrap().as_str().parse()?,
            to: c.name("to").unwrap().as_str().parse()?,
        })
    }
}

fn parse(input: &str) -> Result<(Stacks, Vec<Move>)> {
    let (stacks, procedure) = input
        .trim_end()
        .split("\n\n")
        .tuples()
        .next()
        .wrap_err("Invalid input format")?;

    let stacks: Stacks = stacks.parse()?;
    let moves = procedure
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()?;
    Ok((stacks, moves))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::*;

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\
\n\
move 1 from 2 to 1\n\
move 3 from 1 to 3\n\
move 2 from 2 to 1\n\
move 1 from 1 to 2\n\
"
    }

    #[rstest]
    fn test_moves(input: &str) {
        let (mut stacks, moves) = parse(input).unwrap();

        let mut part1 = stacks.clone();
        part1.execute(&moves).unwrap();
        assert_eq!(part1.top(), "CMZ".to_string());
        stacks.execute_batched(&moves).unwrap();
        assert_eq!(stacks.top(), "MCD".to_string());
    }
}
