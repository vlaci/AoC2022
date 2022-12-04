use std::cmp::Reverse;

use color_eyre::eyre::{Context, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let calories = parse_input(&input)?;
    let max = find_max(&calories);

    println!("The elf having the most calories has {max} calories");

    let top_n = 3;
    let total = find_total(&calories, top_n);
    println!("The top {top_n} elves have {total} calories in total");
    Ok(())
}

type Calories = Vec<Vec<i32>>;

fn parse_input(input: &str) -> Result<Calories> {
    input
        .trim()
        .split("\n\n")
        .map(|e| {
            e.trim()
                .split('\n')
                .map(|n| n.parse().wrap_err_with(|| format!("Not an int {:?}", n)))
                .collect()
        })
        .collect()
}

fn find_max(calories: &Calories) -> i32 {
    find_total(calories, 1)
}

fn find_total(calories: &Calories, top_n: usize) -> i32 {
    let mut sums: Vec<_> = calories.iter().map(|e| e.iter().sum()).collect();
    sums.sort_by_key(|e| Reverse(*e));
    sums.iter().take(top_n).sum()
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
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "}
    }

    #[fixture]
    fn calories() -> Calories {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    #[rstest]
    fn test_parse(input: &str, calories: Calories) {
        let parsed = parse_input(input).unwrap();

        assert_eq!(parsed, calories);
    }

    #[rstest]
    fn test_most_calories(calories: Calories) {
        let max = find_max(&calories);

        assert_eq!(max, 24000);
    }

    #[rstest]
    fn test_top_n_calories(calories: Calories) {
        let total = find_total(&calories, 3);

        assert_eq!(total, 45000);
    }
}
