use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::{cmp::Ordering, str::FromStr};

use color_eyre::{eyre::ContextCompat, Report, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let pairs = parse_pairs(&input)?;
    let correct = count_ordered(&pairs);

    println!("The number of correct pair is {correct}");
    let packets = parse_packets(&input)?;

    let key = get_decoder_key(packets);
    println!("The decoder key is {key}");
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum Packet {
    Literal(u32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn packet(s: &str) -> IResult<&str, Packet> {
            alt((list, literal))(s)
        }

        fn list(s: &str) -> IResult<&str, Packet> {
            map(
                delimited(tag("["), separated_list0(tag(","), packet), tag("]")),
                Packet::List,
            )(s)
        }

        fn literal(s: &str) -> IResult<&str, Packet> {
            map(nom::character::complete::u32, Packet::Literal)(s)
        }

        Ok(all_consuming(packet)(s).map_err(|err| err.to_owned())?.1)
    }
}

fn check_order(left: &Packet, right: &Packet) -> Option<Decision> {
    use Decision::*;
    use Packet::*;

    match (left, right) {
        (Literal(left), Literal(right)) => match left.cmp(right) {
            Ordering::Less => Some(Correct),
            Ordering::Equal => None,
            Ordering::Greater => Some(Incorrect),
        },
        (List(left), List(right)) => {
            left.iter()
                .zip_longest(right.iter())
                .find_map(|pair| match pair {
                    itertools::EitherOrBoth::Both(left, right) => check_order(left, right),
                    itertools::EitherOrBoth::Left(_) => Some(Incorrect),
                    itertools::EitherOrBoth::Right(_) => Some(Correct),
                })
        }
        (left @ Literal(_), right @ List(_)) => check_order(&List(vec![left.clone()]), right),
        (left @ List(_), right @ Literal(_)) => check_order(left, &List(vec![right.clone()])),
    }
}

fn parse_pair(s: &str) -> Result<(Packet, Packet)> {
    let (left, right) = s
        .lines()
        .map(|l| l.parse())
        .next_tuple()
        .wrap_err("Expected two lines")?;
    Ok((left?, right?))
}

fn parse_pairs(s: &str) -> Result<Vec<(Packet, Packet)>> {
    s.trim().split("\n\n").map(parse_pair).collect()
}

fn count_ordered(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| {
            check_order(l, r)
                .map(|o| match o {
                    Decision::Correct => Some(i + 1),
                    _ => None,
                })
                .expect("Undeterminable order found")
        })
        .sum()
}

fn parse_packets(s: &str) -> Result<Vec<Packet>> {
    s.lines()
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_>>()
}

fn get_decoder_key(mut packets: Vec<Packet>) -> usize {
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Literal(2)])]);
    let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Literal(6)])]);
    packets.push(divider_2.clone());
    packets.push(divider_6.clone());

    packets.sort_by(|a, b| check_order(a, b).unwrap().into());

    packets
        .into_iter()
        .positions(|p| p == divider_2 || p == divider_6)
        .map(|p| p + 1)
        .product()
}

#[derive(PartialEq, Debug)]
enum Decision {
    Correct,
    Incorrect,
}

impl From<Decision> for Ordering {
    fn from(d: Decision) -> Self {
        match d {
            Decision::Correct => Ordering::Less,
            Decision::Incorrect => Ordering::Greater,
        }
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
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "}
    }

    #[rstest]
    fn test_packet() {
        let p: Packet = "[[1],4]".parse().unwrap();

        assert_eq!(
            p,
            Packet::List(vec![
                Packet::List(vec![Packet::Literal(1)]),
                Packet::Literal(4)
            ])
        );
    }

    #[rstest]
    #[case("[1,1,3,1,1]\n[1,1,5,1,1]", Decision::Correct)]
    #[case("[[1],[2,3,4]]\n[[1],4]", Decision::Correct)]
    #[case("[9]\n[[8,7,6]]", Decision::Incorrect)]
    #[case("[[4,4],4,4]\n[[4,4],4,4,4]", Decision::Correct)]
    #[case("[7,7,7,7]\n[7,7,7]", Decision::Incorrect)]
    #[case("[]\n[3]", Decision::Correct)]
    #[case("[[[]]]\n[[]]", Decision::Incorrect)]
    #[case(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]",
        Decision::Incorrect
    )]
    fn test_check_order(#[case] pair: &str, #[case] decision: Decision) {
        let (left, right) = parse_pair(pair).unwrap();
        assert_eq!(check_order(&left, &right).unwrap(), decision);
    }

    #[rstest]
    fn test_part1(input: &str) {
        let pairs = parse_pairs(input).unwrap();
        assert_eq!(count_ordered(&pairs), 13);
    }

    #[rstest]
    fn test_part2(input: &str) {
        let packets = parse_packets(input).unwrap();
        let key = get_decoder_key(packets);
        assert_eq!(key, 140);
    }
}
