use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn parse_input(input: &str) -> u32 {

    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+..58
5.592.....
......755.
...$.*....
.664.598..";
        let result = parse_input(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn real_test_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = parse_input(input);
        assert_eq!(result, 4361);
    }
}
