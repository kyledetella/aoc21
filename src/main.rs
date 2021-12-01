fn main() {
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let mut depth_changes = 0;

    for (idx, depth) in depths.iter().enumerate() {
        if idx > 0 {
            if depth > &depths[idx - 1] {
                depth_changes += 1;
            }
        }
    }

    println!("Depth changes: {}", depth_changes);
}
