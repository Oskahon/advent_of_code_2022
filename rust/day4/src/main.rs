fn main() {
    let input = include_str!("../data.txt");

    let mut sum = 0;
    let mut sum2 = 0;

    for line in input.lines() {
        let pairs: Vec<(&str, &str)> = line
            .split(",")
            .map(|s| s.split_once("-")
                .unwrap())
            .collect();

        let first_start = pairs[0].0.parse::<i32>().unwrap();
        let first_end = pairs[0].1.parse::<i32>().unwrap();

        let second_start = pairs[1].0.parse::<i32>().unwrap();
        let second_end = pairs[1].1.parse::<i32>().unwrap();

        let first = second_start - first_start;
        let second = second_end - first_end;

        if first >= 0 && second <= 0 {
            sum += 1;
        } else if first <= 0 && second >= 0 {
            sum += 1;
        }

        let range1 = first_start..=first_end;
        let range2 = second_start..=second_end;

        for n in range1 {
            if range2.contains(&n) {
                sum2 += 1;
                break;
            }
        }
    }

    println!("part1: {}", sum);
    println!("part2: {}", sum2);
}
