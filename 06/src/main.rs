use std::collections::HashSet;

use color_eyre::{eyre::ContextCompat, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let start = start_of_packet(&input).wrap_err("No start-of-packet found")?;
    println!("Start-of-packet found at {start}");
    let start = start_of_message(&input).wrap_err("No start-of-message found")?;
    println!("Start-of-message found at {start}");

    Ok(())
}

fn start_of_packet(buf: &str) -> Option<usize> {
    find_unique_pattern(buf, 4)
}

fn start_of_message(buf: &str) -> Option<usize> {
    find_unique_pattern(buf, 14)
}

fn find_unique_pattern(buf: &str, len: usize) -> Option<usize> {
    for (pos, win) in buf.as_bytes().windows(len).enumerate() {
        if win.iter().collect::<HashSet<_>>().len() == len {
            return Some(pos + len);
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
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6, 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26)]
    fn test_start(#[case] buf: &str, #[case] packet_start: usize, #[case] message_start: usize) {
        assert_eq!(start_of_packet(buf), Some(packet_start));
        assert_eq!(start_of_message(buf), Some(message_start));
    }
}
