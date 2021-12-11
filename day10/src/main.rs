use std::panic;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Valid,
    Incomplete,
    Corrupt
}


fn check_syntax(prev: Option<char>, next: char) -> Option<i64> {
    let expected = match prev {
        Some('(') => ')',
        Some('{') => '}',
        Some('[') => ']',
        Some('<') => '>',
        None => ' ',  // Not expecting a symbol!
        _ => panic!("Unexpected symbol!")
    };

    if next != expected {
        eprint!("Expected {}, but found {} instead.", expected, next);
        let penalty = match next {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unexpected symbol!")
        };
        eprintln!(" Penalty {}", penalty);
        Some(penalty)
    } else {
        None
    }
}


fn main() {
    let lines = include_str!("../instructions.txt").lines();

    let mut score: i64 = 0;
    let mut scores: Vec<i64> = Vec::new();

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut status = Status::Valid;

        'inner: for char in line.chars() {
            // println!("{} + {}", stack.iter().collect::<String>(), char);
            match char {
                w @ ('(' | '{' | '[' | '<') => stack.push(w),
                w @ (')' | '}' | ']' | '>') => {
                    let prev = stack.pop();
                    let penalty = check_syntax(prev, w);
                    if penalty.is_some() {
                        score += penalty.unwrap();
                        // Continue to next line
                        status = Status::Corrupt;
                        break 'inner;
                    }
                }
                _ => panic!("Unexpected character!")
            }
        }

        if (status != Status::Corrupt) & (!stack.is_empty()) {
            status = Status::Incomplete;
            let complete_score = stack
                .iter()
                .rev()
                .fold(0i64, |acc, symbol| {
                    acc * 5 + match symbol {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Unexpected symbol!")
                    }
                });
            scores.push(complete_score);
            println!("Completion score: {}", complete_score);
        }
        println!("{:?}", status);
    }
    println!("Total corruption penalty: {}", score);

    scores.sort();
    println!("Part 2: {}", scores[scores.len() / 2])
}
