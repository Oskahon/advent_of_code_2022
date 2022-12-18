fn main() {
    let input = include_str!("../data.txt");

    let lines = input.lines();
    let mut moves = false;

    for line in  lines {
        if line.is_empty() {
            moves = true;
            break;
        }
        println!();
        let chars = line.chars().collect::<Vec<_>>();
        let index = line.len() / 4;
        for i in 0..=index {
            let c = chars[i * 4 + 1];
            if c == '1' {
                break;
            }
            if c != ' ' {
                print!("{i}:{c} ")
            }
        }

        println!();

        if moves {
            println!("This is a move! \n{}", line);
        } else {
            // println!("{line}");
        }

    }
    
}
