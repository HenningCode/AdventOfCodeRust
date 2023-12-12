use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid filename");
    let result = solve_part1(&input);
    println!("{result}");
}

#[derive(Debug)]
struct Universe {
    map: Vec<Vec<char>>,
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn new(map: Vec<Vec<char>>, galaxies: Vec<Galaxy>) -> Self {
        Self { map, galaxies }
    }

    fn expand_universe(&mut self) {}
}

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

fn num_galaxy_pairs(galaxies: u128) -> u128 {
    factorial(galaxies) / factorial(galaxies - 2) * factorial(2)
}

fn solve_part1(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut galaxies:Vec<Galaxy> = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '#' {
                galaxies.push(Galaxy::new(x, y));
            }
        }
    }

    let universe = Universe::new(map, galaxies);

    println!("{:?}", universe);

    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(solve_part1(input), 374);
    }
}
