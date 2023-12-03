use std::fs;
use std::num::ParseIntError;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(input);
    println!("{result}");
}

fn try_parse_to_int(str: String) -> Result<i32, ParseIntError> {
    let parse = str.parse()?;
    Ok(parse)
}

fn parse_input(input: String) -> i32 {
    let lines: Vec<String> = input.lines().map(|x| String::from(x)).collect();
    let mut result = 0;

    for x in lines {
        let char_vec: Vec<char> = x.chars().collect();
        let mut int_vec: Vec<i32> = Vec::new();
        for char in char_vec {
            let string_char = char.to_string();
            if let Ok(try_parse) = try_parse_to_int(string_char) {
                int_vec.push(try_parse);
            }
        }
        let mut together_int: i32 = 0;
        if let Some(result) = int_vec.first() {
            let value1 = result.to_string().to_owned();
            let value2: String;
            if let Some(result2) = int_vec.last() {
                value2 = result2.to_string().to_owned();
            } else {
                value2 = value1.clone().to_owned();
            }
            let together = format!("{value1}{value2}");
            together_int = together.parse().unwrap();
        }
        result += together_int;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string();
        assert_eq!(parse_input(input), 142);
    }
}
