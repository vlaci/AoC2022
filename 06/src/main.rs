use std::collections::HashSet;

use color_eyre::{eyre::ContextCompat, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let start = start_of_packet(&input).wrap_err("No start-of-packet found")?;
    println!("Start-of-packet found at {start}");

    Ok(())
}

fn start_of_packet(buf: &str) -> Option<usize> {
    for (pos, win) in buf.as_bytes().windows(4).enumerate() {
        if win.iter().collect::<HashSet<_>>().len() == 4 {
            return Some(pos + 4);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_start_of_packet(#[case] buf: &str, #[case] start: usize) {
        assert_eq!(start_of_packet(buf), Some(start));
    }
}
