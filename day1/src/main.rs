fn main() {
    let data = include_str!("../data.txt");

    let mut calories = Vec::new();

    let elves = data.split("\n\n").collect::<Vec<_>>();

    for elf in elves {
        let sum: i32 = elf
            .lines()
            .map(|n| n.parse::<i32>().unwrap())
            .sum();

        calories.push(sum);
    }


    calories.sort();
    calories.reverse();

    let top1 = calories.first().unwrap();
    let top3: i32 = calories.iter().take(3).sum();

    println!("part1: {top1}");
    println!("part2: {top3}");
}
