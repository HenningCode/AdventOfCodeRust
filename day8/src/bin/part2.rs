use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn parse_input(input: &str) -> u32 {
    let (mut instructions, mut map) = ("", "");

    if let Some((temp1, temp2)) = input.split_once("\n\n") {
        (instructions, map) = (temp1, temp2);
    } else if let Some((temp1, temp2)) = input.split_once("\r\n\r\n") {
        (instructions, map) = (temp1, temp2);
    }

    let instructions_vec: Vec<char> = instructions.chars().collect();

    let mut hash_map = HashMap::new();

    for line in map.lines() {
        let (waypoint, decision) = line.split_once("=").unwrap();
        let (left, right) = decision.split_once(',').unwrap();
        let left = left.trim().replace(&['(', ')'], "");
        let right = right.trim().replace(&['(', ')'], "");

        hash_map.insert(waypoint.trim(), (left, right));
    }
    let len = instructions_vec.len();
    let mut start_vec = Vec::new();
    
    for key in hash_map.clone() {
        if key.0.ends_with('A') {
            start_vec.push(key.0)
        }
    }

    println!("{:?}", start_vec);
 
    let (mut left_vec, mut right_vec) = (Vec::new(), Vec::new());

    for start_node in start_vec {
        let (left, right) = hash_map.get(start_node).unwrap().clone();
        left_vec.push(left);
        right_vec.push(right);
    }

    let mut finish = 0;
    let mut i = 0;
    loop {
        let current_instruction = instructions_vec[i % len];     
        // println!("C: {current_instruction}, VecR: {:?}, VecL: {:?}", right_vec, left_vec );

        match current_instruction {
            'L' => {
                for (i, value) in left_vec.clone().iter().enumerate() {
                    
                    if value.ends_with('Z') {
                        finish += 1;
                    }
                    let (left, right) = hash_map.get(value.as_str()).unwrap().clone();
                    left_vec[i] = left;
                    right_vec[i]= right;
                }
                // println!("I {current_instruction} Values Left: {:?}", left_vec);
            }
            'R' => {
                for (i, value) in right_vec.clone().iter().enumerate() {
                    
                    if value.ends_with('Z') {
                        finish += 1;
                    } 
                    let (left, right) = hash_map.get(value.as_str()).unwrap().clone();
                    left_vec[i] = left;
                    right_vec[i]= right;
                }
                // println!("I {current_instruction} Values Right: {:?}", right_vec);
            }
            _ => panic!("Unknow instruction"),
        }
        i += 1;


        if finish == right_vec.len() {
            break;
        }
        println!("{finish}");
        finish = 0;
    }

    i as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(parse_input(input), 6);
    }
}
