use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(input) = read_input("day2.txt") {
        let mut horizontal: i32 = 0;
        let mut vertical: i32 = 0;

        // First pass
        for line in input {
            if let Ok(ln) = line {
                let rule: Vec<&str> = ln.split(" ").collect();
                let num_spaces: i32 = rule[1].parse::<i32>().unwrap();

                match rule[0] {
                    "forward" => horizontal += num_spaces,
                    "down" => vertical += num_spaces,
                    "up" => vertical -= num_spaces,
                    _ => println!("Unknown rule: {}", rule[0]),
                }
            }
        }

        println!(
            "horizontal: {}, vertical: {}\n\nproduct: {}",
            horizontal,
            vertical,
            horizontal * vertical
        )
    }
}
