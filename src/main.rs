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
        let mut depth_changes = 0;
        let mut last_depth: i32 = 0;

        for (idx, line) in input.enumerate() {
            if let Ok(ln) = line {
                if idx > 0 {
                    let depth = ln.parse::<i32>().unwrap();

                    if depth > last_depth {
                        depth_changes += 1;
                    }

                    last_depth = depth;
                }
            }
        }

        println!("Depth changes {}", depth_changes);
    }
}

// fn main() {
//     let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

//     let mut depth_changes = 0;

//     for (idx, depth) in depths.iter().enumerate() {
//         if idx > 0 {
//             if depth > &depths[idx - 1] {
//                 depth_changes += 1;
//             }
//         }
//     }

//     println!("Depth changes: {}", depth_changes);
// }
