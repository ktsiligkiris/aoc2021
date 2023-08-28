use std::collections::HashSet;

fn find_marker(input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(4)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == 4)
        .map(|pos| pos + 4)
}

fn main() {
    dbg!(find_marker(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker(index: usize, input: &str) {
        assert_eq!(Some(index), find_marker(input));
    }
}
