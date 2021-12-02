fn count_increasing<T>(series: &[T]) -> u32
where
    T: PartialOrd,
{
    let mut counter = 0;
    let mut prev_val: Option<&T> = None;

    for value in series {
        if matches!(prev_val, Some(v) if value > v) {
            counter += 1
        }

        prev_val = Some(value);
    }

    counter
}

fn main() {
    let lines = include_str!("../numbers.txt").lines();
    let numbers: Vec<i32> = lines.map(|l| l.parse().unwrap()).collect();

    let part1_counter = count_increasing(&numbers);

    let windows: Vec<i32> = numbers
        .windows(3)
        .map(|w| w.iter().sum())
        .collect();
    let part2_counter = count_increasing(&windows);

    println!("Part 1: The number of increasing values is {}", part1_counter);
    println!("Part 2: The number of increasing values is {}", part2_counter);
}
