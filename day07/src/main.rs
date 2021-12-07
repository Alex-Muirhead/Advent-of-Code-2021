fn tri(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn diff_fuel(positions: &[i32], goal: i32) -> i32 {
    positions
        .iter()
        .enumerate()
        .map(|(n, c)| {
            c * match goal - n as i32 {
                val if val <= 0 => val - 1,
                val => val,
            }
        })
        .sum()
}

fn main() {
    let crabs: Vec<usize> = include_str!("../crabs.txt")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let max_val = crabs.iter().max().unwrap().clone();
    let mut counts: Vec<i32> = vec![0; max_val + 1];

    let mut fuel = 0;

    for crab in crabs {
        counts[crab] += 1;
        fuel += tri(crab as i32);
    }

    for goal in 0..max_val as i32 {
        let diff = diff_fuel(&counts, goal + 1);
        if diff > 0 {
            println!("Goal is {}", goal);
            println!("Fuel required is {}", fuel);
            break;
        }
        fuel += diff;
    }
}
