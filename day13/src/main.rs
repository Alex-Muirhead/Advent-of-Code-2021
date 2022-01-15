use regex::Regex;

fn main() {
    let lines = include_str!("../origami.txt").lines();

    let mut field: Vec<(usize, usize)> = Vec::new();

    let check = Regex::new(r"fold along (x|y)=(\d+?)").unwrap();

    let mut instr = false;
    for line in lines {
        if line == "" {
            instr = true;  // Switch to reading instructions
            continue;
        }

        if instr {
            match check.captures(line) {
                Some(capture) => {
                    print!("{} = ", capture.get(1).unwrap().as_str());
                    println!("{}", capture.get(2).unwrap().as_str().parse::<usize>().unwrap())
                },
                _ => panic!("Invalid instruction")
            }
        } else {
            let coord = line.split_once(',').unwrap();
            let coord = (coord.0.parse().unwrap(), coord.1.parse().unwrap());
            field.push(coord);
        }
    }

    println!("Field: {:?}", field)
}
