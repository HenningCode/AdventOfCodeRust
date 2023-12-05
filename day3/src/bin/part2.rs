use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn get_number_from_range(i: usize, j: usize, line: &String) -> Option<u32> {
    if i == j {
        return None;
    }

    let mut num: String = String::new();
    for k in i..=j {
        let char = line.chars().nth(k).unwrap();
        if char.is_ascii_digit() {
            num.push(char);
        }
    }

    num.parse().ok()
}

fn same_row(current_char_index: usize, line: &String) -> Vec<u32> {
    let line_len_index = line.len() - 1;
    let mut numbers: Vec<u32> = Vec::new();

    // Indices for the left side of the number, bound checking to not get out of bounds
    // 134*.. left_index is the 1 and right index the 4
    let right_index = current_char_index.saturating_sub(1);
    let left_index = current_char_index.saturating_sub(3);

    // first check left of the gear
    if line.chars().nth(right_index).unwrap().is_ascii_digit() {
        if let Some(num) = get_number_from_range(left_index, right_index, &line) {
            numbers.push(num);
        }
    }

    // Indices for the right side of the number, bound checking to not get out of bounds
    // *134.. left_index is the 1 and right index the 4
    let right_index = line_len_index.min(current_char_index + 3);
    let left_index = line_len_index.min(current_char_index + 1);

    // return early if the gear is on the end
    if left_index == current_char_index {
        return numbers;
    }

    // check right of the gear
    if line.chars().nth(left_index).unwrap().is_ascii_digit() {
        if let Some(num) = get_number_from_range(left_index, right_index, &line) {
            numbers.push(num);
        }
    }
    numbers
}

fn row_above_below(current_char_index: usize, line: &String) -> Vec<u32> {
    let line_len_index = line.len() - 1;
    let mut numbers: Vec<u32> = Vec::new();

    // If there is a number directly above or below the gear -
    // there can only be one number in this line
    if line
        .chars()
        .nth(current_char_index)
        .unwrap()
        .is_ascii_digit()
    {
        let left_index = current_char_index.saturating_sub(1);
        let right_index = line_len_index.min(current_char_index + 1);
        
        if let Some(num) = get_number_from_range(left_index, right_index, line) {
            numbers.push(num);
        }
        
    } else {
        // TODO just check left and right for a number.
        
    }

    let left_start_index = current_char_index.saturating_sub(1);
    if line.chars().nth(left_start_index).unwrap().is_ascii_digit() {
        return numbers;
    }

    let left_index = current_char_index;
    let right_index = line_len_index.min(current_char_index + 3);

    if line
        .chars()
        .nth(left_start_index + 1)
        .unwrap()
        .is_ascii_digit()
    {
        if let Some(num) = get_number_from_range(left_index, right_index, &line) {
            numbers.push(num);
        }
    }

    numbers
}

fn unpack_into_tuple(numbers: &Vec<u32>) -> Option<(u32, u32)> {
    if numbers.len() == 2 {
        return Some((numbers[0], numbers[1]));
    }
    return None;
}

fn numbers_around(
    current_char_index: usize,
    current_line_index: usize,
    lines: &Vec<String>,
) -> Option<(u32, u32)> {
    let mut numbers: Vec<u32> = Vec::new();

    numbers.append(&mut same_row(
        current_char_index,
        &lines[current_line_index],
    ));

    // Top line only needs checking for row below
    if current_line_index == 0 {
        numbers.append(&mut row_above_below(
            current_char_index,
            &lines[current_line_index + 1],
        ));
        return unpack_into_tuple(&numbers);

    // Last line only needs checking for row above
    } else if current_line_index == lines.len() {
        numbers.append(&mut row_above_below(
            current_char_index,
            &lines[current_line_index - 1],
        ));
        return unpack_into_tuple(&numbers);
    }

    // All lines in the middle need to be checked above and below
    numbers.append(&mut row_above_below(
        current_char_index,
        &lines[current_line_index + 1],
    ));

    numbers.append(&mut row_above_below(
        current_char_index,
        &lines[current_line_index - 1],
    ));

    unpack_into_tuple(&numbers)
}

fn parse_input(str: &str) -> u32 {
    let lines: Vec<String> = str.lines().map(String::from).collect();
    let mut result: u32 = 0;

    for (i, line) in lines.clone().into_iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            // gear found
            if char == '*' {
                if let Some((num1, num2)) = numbers_around(j, i, &lines) {
                    result += num1 * num2;
                }
            }
        }
    }

    result
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
        assert_eq!(result, 467835);
    }

    #[test]
    fn check_same_line() {
        let input = "467..114..
...*......
..35..633.
......#...
617*112...
.....+..58
5.592.....
......755.
...$.*....
.664.598..";
        let result = parse_input(input);
        assert_eq!(result, 4361);
    }
}
