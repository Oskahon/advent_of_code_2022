use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt")
        .expect("Error reading file");

    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;
    let mut carried = 0;

    for line in data.lines() {
        if line.len() == 0 {
            match carried {
                n if carried > top1 => {
                    top3 = top2;
                    top2 = top1;
                    top1 = n;
                },
                n if carried > top2 => {
                    top3 = top2;
                    top2 = n;
                },
                n if carried > top3 => top3 = n,
                _ => ()
            }
            carried = 0;
            continue;
        }
        let calories: i32 = line.parse().expect("Error parsing");
        carried += calories;
    }

    println!("part1: {}", top1);

    let top_part2 = top1 + top2 + top3;
    println!("part2: {}", top_part2);
}
