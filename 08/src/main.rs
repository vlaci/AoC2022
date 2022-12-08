use std::{iter, str::FromStr};

use color_eyre::{eyre::ContextCompat, Report, Result};
use ndarray::{s, Array2};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let m: Matrix = input.parse()?;

    let visible = count_visible(&m);
    println!("There are {visible} number of trees visible");

    let scenic = scenic(&m);
    println!("The biggest scenic score is {scenic}");

    Ok(())
}

#[derive(PartialEq, Debug)]
struct Matrix(Array2<u8>);

impl std::ops::Deref for Matrix {
    type Target = Array2<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Matrix {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let cols = s.lines().next().wrap_err("Empty input")?.len();
        let rows = s.len() / cols;

        let mut rv = Array2::zeros((rows, cols));
        for (mut row, line) in iter::zip(rv.rows_mut(), s.lines()) {
            for (i, x) in line
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .wrap_err_with(|| format!("Non-numeric character '{c}"))
                })
                .enumerate()
            {
                row[i] = x? as u8;
            }
        }

        Ok(Self(rv))
    }
}

fn count_visible(m: &Matrix) -> usize {
    m.indexed_iter()
        .filter(|&((y, x), current_height)| {
            let col = m.column(x);
            let row = m.row(y);

            [
                row.slice(s![..x;-1]),
                row.slice(s![x + 1..]),
                col.slice(s![..y;-1]),
                col.slice(s![y + 1..]),
            ]
            .iter()
            .map(|it| it.iter().all(|t| t < current_height))
            .any(|t| t)
        })
        .count()
}
fn scenic(m: &Matrix) -> usize {
    m.indexed_iter()
        .map(|((y, x), current_height)| {
            let col = m.column(x);
            let row = m.row(y);
            [
                row.slice(s![..x; -1]),
                row.slice(s![x + 1..]),
                col.slice(s![..y; -1]),
                col.slice(s![y + 1..]),
            ]
            .iter()
            .map(|ax| {
                ax.iter()
                    .position(|tree| tree >= current_height)
                    .map_or_else(|| ax.dim(), |p| p + 1)
            })
            .product()
        })
        .max()
        .unwrap()
}

impl From<Array2<u8>> for Matrix {
    fn from(a: Array2<u8>) -> Self {
        Matrix(a)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use ndarray::arr2;
    use pretty_assertions::assert_eq;
    use rstest::*;

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        indoc! {"
            30373
            25512
            65332
            33549
            35390
        "}
    }

    #[fixture]
    fn matrix() -> Matrix {
        arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ])
        .into()
    }

    #[rstest]
    fn test_parsing(input: &str, matrix: Matrix) {
        let m: Matrix = input.parse().unwrap();

        assert_eq!(m, matrix);
    }

    #[rstest]
    fn test_count_visible(matrix: Matrix) {
        assert_eq!(count_visible(&matrix), 21);
    }
    #[rstest]
    fn test_scenic_score(matrix: Matrix) {
        assert_eq!(scenic(&matrix), 8);
    }
}
