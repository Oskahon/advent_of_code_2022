use itertools::Itertools;

fn main() {
    let data = include_str!("../data.txt");

    println!("part1: {}", part1(data));
    println!("part2: {}", part2(data));
}

fn part1(data: &str) -> i32 {
    let mut sum: i32 = 0;
    let a = b'a' as i32;

    for line in data.lines() {
        let (first_comp, second_comp) = line
            .split_at(line.len() / 2);
        
        let item_in_both = first_comp
            .chars()
            .filter(|c| second_comp.contains(c.to_owned()))
            .next()
            .unwrap();

        let mut priority: i32 = 0;
        
        if item_in_both.is_uppercase() {
            priority += item_in_both as i32 - a + 27 + 32;
        } else {
            priority += item_in_both as i32 - a + 1;
        }

        sum += priority;
    }

    sum
}

fn part2(data: &str) -> i32 {
    let a = b'a' as i32;
    let mut sum: i32 = 0;

    for line in &data.lines().chunks(3) {
        let group = line.collect_vec();

        let item_in_all = group[0]
            .chars()
            .filter(|c| group[1].contains(c.to_owned()))
            .filter(|c| group[2].contains(c.to_owned()))
            .next()
            .unwrap();

        let mut priority: i32 = 0;

        if item_in_all.is_uppercase() {
            priority += item_in_all as i32 - a + 27 + 32;
        } else {
            priority += item_in_all as i32 - a + 1;
        }

        sum += priority;
    }

    sum
}
