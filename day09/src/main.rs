fn get_neighbours(map: &Vec<Vec<i32>>, index: (isize, isize), bounds: (usize, usize)) -> Vec<i32> {
    let mut neighbours = Vec::new();

    for vert in -1..=1 {
        for horz in -1..=1 {
            if !((vert == 0) ^ (horz == 0)) {
                continue;
            }

            let mut next_index = index.clone();
            next_index.0 += vert;
            next_index.1 += horz;

            if (next_index.0 >= 0)
                & (next_index.1 >= 0)
                & (next_index.0 < bounds.0 as isize)
                & (next_index.1 < bounds.1 as isize)
            {
                let value = map[next_index.0 as usize][next_index.1 as usize];
                neighbours.push(value);
            }
        }
    }

    neighbours
}

fn main() {
    let lines = include_str!("../map.txt").lines();

    let height_map: Vec<Vec<i32>> = lines
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let height = height_map.len();
    let width = height_map.first().unwrap().len();

    let mut local_minima: Vec<i32> = Vec::new();
    let mut to_check: Vec<(isize, isize)> = Vec::new();

    for y in 0..height as isize {
        for x in 0..width as isize {
            let value = height_map[y as usize][x as usize];
            let neighbours = get_neighbours(&height_map, (y, x), (height, width));
            if neighbours.iter().all(|n| n > &value) {
                local_minima.push(value);
                to_check.push((x, y));
            }
        }
    }

    println!(
        "Part 1 answer: {}",
        local_minima.iter().map(|v| v + 1).sum::<i32>()
    );

    let mut basin_sizes: Vec<i32> = vec![1; local_minima.len() + 1];

    let mut basin_map: Vec<Vec<i32>> = vec![vec![0; width]; height];
    for (n, &(x, y)) in to_check.iter().enumerate() {
        basin_map[y as usize][x as usize] = n as i32 + 1;
    }

    while !to_check.is_empty() {
        let (x, y) = to_check.pop().unwrap();
        let basin_num = basin_map[y as usize][x as usize];

        for vert in -1..=1 {
            for horz in -1..=1 {
                if !((vert == 0) ^ (horz == 0)) {
                    continue;
                }

                let mut next_index = (y, x).clone();
                next_index.0 += vert;
                next_index.1 += horz;

                if (next_index.0 >= 0)
                    & (next_index.1 >= 0)
                    & (next_index.0 < height as isize)
                    & (next_index.1 < width as isize)
                {
                    let value = height_map[next_index.0 as usize][next_index.1 as usize];
                    if basin_map[next_index.0 as usize][next_index.1 as usize] != 0 {
                        continue;
                    }

                    if value != 9 {
                        basin_map[next_index.0 as usize][next_index.1 as usize] = basin_num;
                        to_check.push((next_index.1, next_index.0));
                        basin_sizes[basin_num as usize] += 1;
                    } else {
                        basin_map[next_index.0 as usize][next_index.1 as usize] = 9;
                    }
                }
            }
        }
    }

    basin_sizes.sort();

    println!(
        "Basin sizes: {:?}",
        basin_sizes.into_iter().rev().take(3).fold(1, |acc, mul| acc * mul)
    );
}
