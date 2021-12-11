fn main() {
    let lines = include_str!("../octopuses.txt").lines();

    let mut octopuses: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        octopuses.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut flash_count = 0;

    // for line in &octopuses {
    //     for val in line {
    //         print!("{}", val);
    //     }
    //     println!("")
    // }
    // println!("\n----------\n");

    // let mut flashes: Vec<(usize, usize)> = Vec::new();
    for step in 0..1000 {
        let mut flashes: Vec<(usize, usize)> = octopuses
            .iter_mut()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter_mut().enumerate().filter_map(move |(j, oct)| {
                    *oct += 1;
                    if *oct > 9 {
                        *oct = 0;
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect();

        while !flashes.is_empty() {
            flash_count += 1;
            let (index_i, index_j) = flashes.pop().unwrap();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if (dx == 0) & (dy == 0) {
                        continue;
                    }
                    let n_index = (index_i as isize + dy, index_j as isize + dx);

                    if (n_index.0 >= 0) & (n_index.0 < 10) & (n_index.1 >= 0) & (n_index.1 < 10) {
                        if octopuses[n_index.0 as usize][n_index.1 as usize] == 0 {
                            continue;
                        }
                        octopuses[n_index.0 as usize][n_index.1 as usize] += 1;
                        if octopuses[n_index.0 as usize][n_index.1 as usize] > 9 {
                            flashes.push((n_index.0 as usize, n_index.1 as usize));
                            octopuses[n_index.0 as usize][n_index.1 as usize] = 0;
                        }
                    }
                }
            }
        }

        if octopuses.iter().all(|row| {
            row.iter().all(|&v| v == 0)
        }) {
            println!("Synced at {}", step+1);
            break;
        }


    }
    for line in &octopuses {
        for val in line {
            print!("{}", val);
        }
        println!("")
    }
    println!("\n----------\n");

    println!("Total flashes: {}", flash_count)
}
