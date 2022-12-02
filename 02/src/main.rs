use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let game = parse(&input);
    let total = score(&game);

    println!("The total score is {total}");

    let total = score(&to_game(&parse_outcome(&input)));
    println!("The total score is {total}");

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<(Shape, Shape)> {
    input
        .trim()
        .lines()
        .map(|l| {
            (
                l.chars().next().unwrap().into(),
                l.chars().nth(2).unwrap().into(),
            )
        })
        .collect()
}

fn parse_outcome(input: &str) -> Vec<(Shape, Outcome)> {
    input
        .trim()
        .lines()
        .map(|l| {
            (
                l.chars().next().unwrap().into(),
                l.chars().nth(2).unwrap().into(),
            )
        })
        .collect()
}

fn to_game(out: &[(Shape, Outcome)]) -> Vec<(Shape, Shape)> {
    use Outcome::*;
    use Shape::*;

    out.iter()
        .map(|pair| match pair {
            (Rock, Lose) => (Rock, Scissors),
            (Rock, Draw) => (Rock, Rock),
            (Rock, Win) => (Rock, Paper),
            (Paper, Lose) => (Paper, Rock),
            (Paper, Draw) => (Paper, Paper),
            (Paper, Win) => (Paper, Scissors),
            (Scissors, Lose) => (Scissors, Paper),
            (Scissors, Draw) => (Scissors, Scissors),
            (Scissors, Win) => (Scissors, Rock),
        })
        .collect()
}

fn score(game: &[(Shape, Shape)]) -> i32 {
    use Shape::*;
    game.iter()
        .map(|pair| match pair {
            (Rock, Rock) => 1 + 3,
            (Rock, Paper) => 2 + 6,
            (Rock, Scissors) => 3,
            (Paper, Rock) => 1,
            (Paper, Paper) => 2 + 3,
            (Paper, Scissors) => 3 + 6,
            (Scissors, Rock) => 1 + 6,
            (Scissors, Paper) => 2,
            (Scissors, Scissors) => 3 + 3,
        })
        .sum()
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
            A Y
            B X
            C Z
        "}
    }

    #[rstest]
    fn test_first(input: &str) {
        let game = parse(input);

        assert_eq!(
            game,
            vec![
                (Shape::Rock, Shape::Paper),
                (Shape::Paper, Shape::Rock),
                (Shape::Scissors, Shape::Scissors),
            ]
        );

        let total = score(&game);
        assert_eq!(total, 15);
    }

    #[rstest]
    fn test_second(input: &str) {
        let result = parse_outcome(input);

        assert_eq!(
            result,
            vec![
                (Shape::Rock, Outcome::Draw),
                (Shape::Paper, Outcome::Lose),
                (Shape::Scissors, Outcome::Win),
            ]
        );

        let game = to_game(&result);
        assert_eq!(
            game,
            vec![
                (Shape::Rock, Shape::Rock),
                (Shape::Paper, Shape::Rock),
                (Shape::Scissors, Shape::Rock),
            ]
        );

        let total = score(&game);
        assert_eq!(total, 12);
    }
}
