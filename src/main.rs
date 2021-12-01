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
    if let Ok(input) = read_input("day1.txt") {
        let mut depths: Vec<i32> = Vec::new();

        // First pass
        for line in input {
            if let Ok(ln) = line {
                depths.push(ln.parse::<i32>().unwrap());
            }
        }

        let mut windows: Vec<i32> = Vec::new();

        for (idx, depth) in depths.iter().enumerate() {
            if idx < depths.len() - 2 {
                windows.push(depth.to_owned() + depths[idx + 1] + depths[idx + 2]);
                // println!("[{}, {}, {}]", depth, depths[idx + 1], depths[idx + 2]);
            }
        }

        let mut depth_changes = 0;
        let mut last_depth: &i32 = &0.to_owned();

        for window in windows.iter() {
            println!("{}, {}", window, last_depth);

            if window > last_depth {
                depth_changes += 1;
            }

            last_depth = window;
        }

        println!("Depth changes {}", depth_changes - 1);
    }
}
