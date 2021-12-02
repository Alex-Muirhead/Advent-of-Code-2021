struct SimpleSubmarine {
    depth: i32,
    position: i32,
}

impl SimpleSubmarine {
    fn steer(&mut self, heading: &str, distance: i32) {
        match heading {
            "forward" => self.position += distance,
            "up" => self.depth -= distance,
            "down" => self.depth += distance,
            _ => panic!("Unexpected direction!"),
        };
    }

    fn report(&self) -> (i32, i32) {
        (self.depth, self.position)
    }
}

struct ComplexSubmarine {
    depth: i32,
    position: i32,
    aim: i32
}

impl ComplexSubmarine {
    fn steer(&mut self, heading: &str, distance: i32) {
        match heading {
            "forward" => {
                self.position += distance;
                self.depth += self.aim * distance;
            },
            "up" => self.aim -= distance,
            "down" => self.aim += distance,
            _ => panic!("Unexpected direction!"),
        };
    }

    fn report(&self) -> (i32, i32) {
        (self.depth, self.position)
    }
}

fn main() {
    let lines = include_str!("../course.txt").lines();

    let mut part1_sub = SimpleSubmarine{ depth: 0, position: 0 };
    let mut part2_sub = ComplexSubmarine{ depth: 0, position: 0, aim: 0 };

    for line in lines {
        if let Some((dir, num)) = line.split_once(' ') {
            let dist: i32 = num.parse().unwrap();

            part1_sub.steer(dir, dist);
            part2_sub.steer(dir, dist);
        }
    }

    let (depth, position) = part1_sub.report();
    println!("Submarine 1 is at pos {} and depth  {}", position, depth);
    println!("Part 1 problem answer: {}", position * depth);

    let (depth, position) = part2_sub.report();
    println!("Submarine 2 is at pos {} and depth  {}", position, depth);
    println!("Part 2 problem answer: {}", position * depth);
}
