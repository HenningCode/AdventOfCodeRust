use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file"); 
    let result = parse_input(&input);
    println!("{result}");
}

fn symbols_around(current_line_index:usize, current_char_index:usize, number_len:usize, lines:&Vec<String>) -> bool {
 
    let mut indices_above:Vec<usize> = Vec::new();
    let mut indices_same:Vec<usize> = Vec::new();
    let mut indices_below:Vec<usize> = Vec::new();


    println!("Index:{current_char_index}, Len:{number_len}");
    
    if current_char_index == 0 {

    } else if current_char_index == lines.len() {

    } 



    true
}

fn parse_input(str: &str) -> u32 {
    let lines:Vec<String> = str.lines().map(|x| String::from(x)).collect();
    let mut result: u32 = 0;
    let mut current_number:Vec<char> = Vec::new();

    for (i, line) in lines.clone().into_iter().enumerate() {
        for (j, char) in line.chars().into_iter().enumerate() {
            // TODO wrong index for eol checking
            if char == '.' && !current_number.is_empty() || j+1 == line.len() && !current_number.is_empty() {
                if symbols_around(i, j, current_number.len(), &lines) {
                    let temp:String = current_number.clone().into_iter().collect();
                    println!("Number: {temp}");
                    result += temp.parse::<u32>().unwrap();
                }
                current_number.clear();
            }
      
            if char.is_ascii_digit() {
                current_number.push(char);
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
