use std::fmt;
use std::iter;

const BOARD_SIZE: usize = 5;

#[derive(Default, Debug)]
struct Board {
    numbers: Vec<u32>,
    marked: Vec<bool>,
    won: bool
}

impl Board {
    fn add_row(&mut self, row: &mut Vec<u32>) {
        self.numbers.append(row);
        self.marked.append(&mut vec![false; BOARD_SIZE]);
    }

    fn contains(&self, num: u32) -> Option<usize> {
        self.numbers
            .iter()
            .enumerate()
            .find_map(|(i, &v)| if v == num { Some(i) } else { None })
    }

    fn check_row(&self, row: usize) -> bool {
        self.marked
            .chunks(BOARD_SIZE)
            .skip(row)
            .next()
            .unwrap()
            .iter()
            .all(|&x| x)
    }

    fn check_col(&self, col: usize) -> bool {
        self.marked
            .chunks(BOARD_SIZE)
            .all(|row| *row.iter().skip(col).next().unwrap())
    }

    fn answer(&self, num: u32) -> u32 {
        num * self.numbers
            .iter()
            .zip(self.marked.iter())
            .filter(|(_, marked)| !**marked)
            .map(|(val, _)| val)
            .sum::<u32>()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.numbers.chunks(5) {
            for val in row {
                write!(f, "{:>3}", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut lines = include_str!("../game.txt").lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut new_board = Board::default();

    // Skip first newline, and add newline to end
    for line in lines.skip(1).chain(iter::once("")) {
        // New line indicates new board
        if line.is_empty() {
            boards.push(new_board);
            new_board = Board::default();
        } else {
            let mut row: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            new_board.add_row(&mut row);
        }
    }

    // 'outer: for num in numbers {
    //     for board in &mut boards {
    //         if let Some(index) = board.contains(num) {
    //             board.marked[index] = true;
    //             let (row, col) = (index / BOARD_SIZE, index % BOARD_SIZE);
    //             if board.check_col(col) || board.check_row(row) {
    //                 println!("Board won\n{}", board);
    //                 println!("The answer is: {}", board.answer(num));
    //                 break 'outer;
    //             }
    //         }
    //     }
    // }

    let mut counter = 0;
    let total_boards = boards.len();

    'outer: for num in numbers {
        for board in &mut boards {
            if let Some(index) = board.contains(num) {
                board.marked[index] = true;
                let (row, col) = (index / BOARD_SIZE, index % BOARD_SIZE);
                if !board.won && (board.check_col(col) || board.check_row(row)) {
                    counter += 1;
                    board.won = true;
                    if counter == total_boards {
                        println!("The last board to win is \n{}", board);
                        println!("The answer is: {}", board.answer(num));
                        break 'outer;
                    }
                }
            }
        }
    }


}
