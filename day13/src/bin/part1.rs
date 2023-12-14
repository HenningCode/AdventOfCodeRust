use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = solve(&input);
    println!("{result}");
}

fn solve(input: &str) -> u64 {
    let mut data: Vec<Vec<String>> = Vec::new();
    let mut block: Vec<String> = Vec::new();
    for line in input.lines() {
        if line == "" {
            data.push(block.clone());
            block.clear();
        } else {
            block.push(line.to_string());
        }
    }
    data.push(block);

    let mut result = 0;
    for block in data {
        let horizontal = check_horizontal_mirror(&block);
        if horizontal.0 {
            result += horizontal.1;
        } else {
            let vertical = check_vertical_mirror(&block);
            if vertical.0 {
                result += vertical.1 * 100;
            } else {
                panic!("Has to have a mirrow");
            }
        }
    }

    result as u64
}

fn check_horizontal_mirror(input: &Vec<String>) -> (bool, i32) {
    let mut split = -1;
    for i in 1..input.len() {
        if input[i] == input[i - 1] {
            split = i as i32;
        }
    }

    if split == -1 {
        return (false, split);
    }

    if input.len() as i32 - split > split {
        for (i, j) in (0..split - 1).rev().enumerate() {
            if input[j as usize] != input[(split as usize) + i + 1] {
                return (false, split);
            }
        }
    } else {
        for (i, j) in (split + 1..(input.len() as i32)).enumerate() {
            if input[j as usize] != input[(split as usize) - 2 - i] {
                return (false, split);
            }
        }
    }

    (true, split + 1)
}

fn check_vertical_mirror(input: &Vec<String>) -> (bool, i32) {
    let mut split = -1;

    for i in 1..input[0].len() {
        let mut found = true;
        for j in 0..input.len() {
            if input[j].chars().nth(i) != input[j].chars().nth(i - 1) {
                found = false;
                break;
            }
        }

        if found {
            split = i as i32;
            break;
        }
    }
    println!("Found one possible");

    (true, split)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(solve(input), 405);
    }
}
