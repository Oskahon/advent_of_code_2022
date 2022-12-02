use std::collections::HashMap;

fn main() {
    let data = include_str!("../data.txt");
    let mut points = 0;
    let mut points2 = 0;

    for line in data.lines() {
        let round = line.split(' ').collect::<Vec<_>>();
        points += compare(round[0], round[1]);
        points2 += add_points(round[0], round[1]);
    }

    println!("part1: {}", points);
    println!("part2: {}", points2);
}

fn compare(a: &str, b: &str) -> i64 {
    let strat = HashMap::from([
        ("A", 0),
        ("B", 1),
        ("C", 2),
        ("X", 0),
        ("Y", 1),
        ("Z", 2),
    ]);

    let a = strat.get(a).unwrap();
    let b = strat.get(b).unwrap();

    let result: i64 = a - b;
    let mut points: i64 = b + 1;

    match result {
        2 | -1 => points += 6,
        0 => points += 3,
        _ => (),
    }

    points
}

fn add_points(a: &str, b: &str) -> i64 {
    let strat: HashMap<&str, i64> = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);
    
    let mut points = *strat.get(b).unwrap();

    match b {
        "X" => {
            match a {
                "A" => points += 3,
                "B" => points += 1,
                "C" => points += 2,
                _ => (),
            }
        }
        "Y" => {
            match a {
                "A" => points += 1,
                "B" => points += 2,
                "C" => points += 3,
                _ => (),
            }
        }
        "Z" => {
            match a {
                "A" => points += 2,
                "B" => points += 3,
                "C" => points += 1,
                _ => (),
            }
        }
        _ => (),
    }

    points
}
