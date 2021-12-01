use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "numbers.txt";
    let file = File::open(filename).expect("File cannot be opened");
    let lines = BufReader::new(file).lines();

    let numbers: Vec<i32> = lines
        .map(|line| {
            line.unwrap()
                .trim()
                .parse()
                .expect("Could not parse number")
        })
        .collect();

    let mut counter = 0;
    let mut prev_sum: Option<i32> = None;

    for slice in numbers.windows(3) {
        let sum = slice.iter().sum();

        if matches!(prev_sum, Some(prev) if sum > prev) {
            counter += 1;
        }

        prev_sum = Some(sum);
    }

    println!("The number of increased value is {}", counter);
}
