/// Check that all characters of `second` are contained within `first`
fn contains_unordered_str(first: &str, second: &str) -> bool {
    second.chars().all(|c| first.contains(c))
}

/// Check that strings `first` and `second` are contained within each other
fn compare_unordered_str(first: &str, second: &str) -> bool {
    let check_one = contains_unordered_str(first, second);
    let check_two = contains_unordered_str(second, first);
    check_one & check_two
}

fn main() {
    let lines = include_str!("../sequence.txt").lines();
    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in lines {
        let (observation, output) = line.split_once(" | ").unwrap();

        const EMPTY: Vec<&str> = Vec::new(); // To enable Copy
        let mut bin: [Vec<&str>; 8] = [EMPTY; 8];
        let mut digits = [""; 10];

        for pattern in observation.split(' ') {
            let length = pattern.len();
            bin[length].push(pattern);
        }

        // Start conversion of unambiguous values first
        digits[1] = bin[2].first().unwrap();
        digits[4] = bin[4].first().unwrap();
        digits[7] = bin[3].first().unwrap();
        digits[8] = bin[7].first().unwrap();

        for pattern in output.split(' ') {
            let finding = digits.iter().enumerate().find_map(|(val, &d)| {
                if compare_unordered_str(d, pattern) {
                    Some(val)
                } else {
                    None
                }
            });
            if let Some(_value) = finding {
                part1_total += 1;
            }
        }

        // Process rest of values
        let where_nine = bin[6]
            .iter()        // 9 contains 4
            .position(|&s| contains_unordered_str(s, digits[4]))
            .unwrap();
        digits[9] = bin[6].remove(where_nine);

        let where_zero = bin[6]
            .iter()        // 0 contains 1
            .position(|&s| contains_unordered_str(s, digits[1]))
            .unwrap();
        digits[0] = bin[6].remove(where_zero);
        digits[6] = bin[6].first().unwrap();

        let where_three = bin[5]
            .iter()        // 3 contains 1
            .position(|&s| contains_unordered_str(s, digits[1]))
            .unwrap();
        digits[3] = bin[5].remove(where_three);

        let where_five = bin[5]
            .iter()        // 6 contains 5
            .position(|&s| contains_unordered_str(digits[6], s))
            .unwrap();
        digits[5] = bin[5].remove(where_five);
        digits[2] = bin[5].first().unwrap();

        let mut line_value = 0;
        for pattern in output.split(' ') {
            let finding = digits.iter().enumerate().find_map(|(val, &d)| {
                if compare_unordered_str(d, pattern) {
                    Some(val)
                } else {
                    None
                }
            });
            if let Some(value) = finding {
                line_value = line_value * 10 + value;
            }
        }
        part2_total += line_value;
    }

    println!("A total of {} unique values were seen", part1_total);
    println!("The sum of all values is {}", part2_total);
}

#[test]
fn test_comparison() {
    let first = "acedgfb";
    let second = "fdgacbe";
    assert!(compare_unordered_str(first, second))
}
