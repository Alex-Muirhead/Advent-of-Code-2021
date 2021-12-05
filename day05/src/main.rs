use std::{num::ParseIntError, str::FromStr};

#[derive(PartialEq, PartialOrd, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn max(&self, other: &Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        Self { x, y }
    }

    fn min(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        Self { x, y }
    }

    fn index(&self, max_coord: &Self) -> usize {
        (self.y * (max_coord.x + 1) + self.x) as usize
    }
}

impl FromStr for Coord {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.trim().parse()?,
            y: y.trim().parse()?,
        })
    }
}

#[derive(Debug)]
struct Vent {
    from: Coord,
    to: Coord,
}

impl Vent {
    fn max(&self) -> Coord {
        self.from.max(&self.to)
    }

    fn min(&self) -> Coord {
        self.from.min(&self.to)
    }
}

fn main() {
    let lines = include_str!("../lines.txt").lines();

    let mut vents: Vec<Vent> = Vec::new();

    for line in lines {
        let (from, to) = line.split_once(" -> ").unwrap();

        let from = Coord::from_str(from).unwrap();
        let to = Coord::from_str(to).unwrap();

        let new_vent = Vent { from, to };
        vents.push(new_vent);
    }

    let max_coord = vents
        .iter()
        .fold(Coord { x: 0, y: 0 }, |acc, mul| acc.max(&mul.max()));

    let mut map: Vec<u32> = vec![0; ((max_coord.x+1) * (max_coord.y+1)) as usize];

    for vent in vents {
        if vent.from.x == vent.to.x {
            // Vertical vent
            for y in vent.min().y..=vent.max().y {
                let coord = Coord{ y, ..vent.from };
                map[coord.index(&max_coord)] += 1;
            }
        } else if vent.from.y == vent.to.y {
            // Horizontal vent
            for x in vent.min().x..=vent.max().x {
                let coord = Coord{ x, ..vent.from };
                map[coord.index(&max_coord)] += 1;
            }
        } else {
            let x_diff = vent.to.x as i32 - vent.from.x as i32;
            let y_diff = vent.to.y as i32 - vent.from.y as i32;

            if x_diff.abs() != y_diff.abs() {
                panic!("This vent is not 45 deg!")
            }

            let x_sign = x_diff.signum();
            let y_sign = y_diff.signum();

            let (mut x, mut y) = (vent.from.x as i32, vent.from.y as i32);

            for _ in 0..=x_diff.abs() {
                let coord = Coord { x: x as u32, y: y as u32 };
                map[coord.index(&max_coord)] += 1;
                x += x_sign;
                y += y_sign;
            }
        }
    }

    // for row in map.chunks((max_coord.x + 1) as usize) {
    //     for val in row {
    //         print!("{:>3}", val);
    //     }
    //     println!("");
    // }

    let danger_spots = map.iter().filter(|&&v| v >= 2).count();
    println!("Number of dangerous spots: {}", danger_spots);
}
