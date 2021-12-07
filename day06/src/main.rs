const REPR_AGE: usize = 7;

fn main() {
    let line = include_str!("../fish.txt");

    let mut old_fish = vec![0u64; REPR_AGE];
    let mut new_fish = vec![0u64; REPR_AGE + 2];

    for day in line.split(',').map(|f| f.parse::<usize>().unwrap()) {
        old_fish[day+1] += 1;
    }

    for day in 1..=256 {
        let small_cycle = day % REPR_AGE;
        let large_cycle = day % (REPR_AGE + 2);

        old_fish[small_cycle] += new_fish[large_cycle];
        new_fish[large_cycle]  = old_fish[small_cycle];
    }

    println!("After 256 days there are {} fish",
        old_fish.iter().sum::<u64>() + new_fish.iter().sum::<u64>());
}
