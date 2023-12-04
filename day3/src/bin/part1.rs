use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file"); 
    let result = parse_input(&input);
    println!("{result}");
}

fn row_above (current_char_index:usize, number_len:usize, line: &String) -> bool {

    true
}

fn same_row (current_char_index:usize, number_len:usize, line: &String) -> bool {
    // Line length reduced by one because indexing starts at 0
    let line_length = line.len()-1; 

    // Number is first on the line 533..
    // check if next is a symbol
    if current_char_index == number_len-1 {
        return line.chars().nth(current_char_index+1).unwrap() != '.';

    } 
    // Number is last on the line ...533
    // check if previous is a symbol
    if current_char_index == line_length {
        return line.chars().nth(line_length-number_len).unwrap() != '.';
    }

    // middle of the line
    // ...555...
    let mut found = false;
    found |= line.chars().nth(current_char_index+1).unwrap() != '.';
    found |= line.chars().nth(current_char_index-number_len).unwrap() != '.';
    found
}

fn row_below (current_char_index:usize, number_len:usize, line: &String) -> bool {

    true
}

fn symbols_around(current_line_index:usize, current_char_index:usize, number_len:usize, lines:&Vec<String>) -> bool {
    let mut around = false;
    around |= same_row(current_char_index, number_len, &lines[current_line_index]);
    if current_line_index == 0 {
        around |= row_below();
    } else if current_line_index == lines.len() {
        around |= row_above();
    } else {
        around |= row_above();
        around |= row_below();
    }
    around
}

fn parse_input(str: &str) -> u32 {
    let lines:Vec<String> = str.lines().map(|x| String::from(x)).collect();
    let mut result: u32 = 0;
    let mut current_number:Vec<char> = Vec::new();

    for (i, line) in lines.clone().into_iter().enumerate() {
        for (j, char) in line.chars().into_iter().enumerate() {                  
            if char.is_ascii_digit() {
                current_number.push(char);
            }

            if !char.is_ascii_digit() && !current_number.is_empty() {
                if symbols_around(i, j-1, current_number.len(), &lines) {
                    let temp:String = current_number.clone().into_iter().collect();
                    println!("Index:{}, Len:{} Number: {temp}",j-1,current_number.len());
                    result += temp.parse::<u32>().unwrap();
                }
                current_number.clear();
            }

            if j == line.len()-1 && !current_number.is_empty() {
                if symbols_around(i, j, current_number.len(), &lines) {
                    let temp:String = current_number.clone().into_iter().collect();
                    println!("Index:{}, Len:{} Number: {temp}",j,current_number.len());
                    result += temp.parse::<u32>().unwrap();
                }
                current_number.clear();
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
        assert_eq!(result,4361);
    }
}
