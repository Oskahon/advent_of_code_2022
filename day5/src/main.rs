fn main() {
    let input = include_str!("../testi.txt");

    let lines = input.lines();
    let mut moves = false;

    for line in  lines {
        if line.is_empty() {
            moves = true;
            break;
        }
        println!();
        let chars = line.chars().collect::<Vec<_>>();
        let index = line.len() / 3;
        for i in 0..index {
            let c = chars[i * 4 + 1];
            if c != ' ' {
                print!("{}", chars[i * 4 + 1])
            }
        }

        println!();

        if moves {
            println!("This is a move! \n{}", line);
        } else {
            println!("{line}");
        }

    }
    
}
