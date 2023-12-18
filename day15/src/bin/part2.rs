use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = solve(&input);
    println!("{result}");
}

fn solve(input: &str) -> u64 {

    4
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_input(){
        let input = "";

        assert_eq!(solve(input),4);
    }
}