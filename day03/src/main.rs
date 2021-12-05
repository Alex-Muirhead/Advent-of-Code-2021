fn from_str_bits(string: &str) -> Vec<u8> {
    string
        .chars()
        .map(|c| c.to_digit(2).expect("Not a binary digit!") as u8)
        .collect()
}

fn from_vec_bits(bits: &[u8]) -> u32 {
    bits.into_iter().fold(0, |acc, &mul| (acc << 1) | mul as u32)
}

fn main() {
    let lines = include_str!("../diagnostics.txt").lines();

    let diagnostics: Vec<Vec<u8>> = lines.map(from_str_bits).collect();

    let diagnostic_length = diagnostics.len();
    let word_length = diagnostics[0].len();

    // Part 1 calculations

    let mut gamma_bits: Vec<u8> = Vec::new();
    let mut epsilon_bits: Vec<u8> = Vec::new();

    for index in 0..word_length {
        let bit_count: u32 = diagnostics.iter().map(|w| w[index] as u32).sum();

        let val = (2 * bit_count > diagnostic_length as u32) as u8;
        gamma_bits.push(val);
        epsilon_bits.push(1 - val);
    }

    let gamma = from_vec_bits(&gamma_bits);
    let epsilon = from_vec_bits(&epsilon_bits);

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    println!("Part 1 answer: {}", gamma * epsilon);

    // Oxygen calculations

    let mut oxygen_diagnostics = diagnostics.clone();

    for index in 0..word_length {
        let ones_count: u32 = oxygen_diagnostics.iter().map(|w| w[index] as u32).sum();

        let zeros_count = oxygen_diagnostics.len() as u32 - ones_count;
        let most_common = (ones_count >= zeros_count) as u8;

        oxygen_diagnostics = oxygen_diagnostics
            .into_iter()
            .filter(|w| w[index] == most_common)
            .collect();

        if oxygen_diagnostics.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = from_vec_bits(&oxygen_diagnostics[0]);
    println!("Oxygen {:?}", oxygen_generator_rating);

    // CO2 scrubber calculations

    let mut scrubber_diagnostics = diagnostics.clone();

    for index in 0..word_length {
        let ones_count: u32 = scrubber_diagnostics.iter().map(|w| w[index] as u32).sum();

        let zeros_count = scrubber_diagnostics.len() as u32 - ones_count;
        let least_common = (ones_count < zeros_count) as u8;

        scrubber_diagnostics = scrubber_diagnostics
            .into_iter()
            .filter(|w| w[index] == least_common)
            .collect();

        if scrubber_diagnostics.len() == 1 {
            break;
        }
    }

    let co2_scrubber_rating = from_vec_bits(&scrubber_diagnostics[0]);
    println!("CO2 scrubber {:?}", co2_scrubber_rating);

    println!("Part 2 answer: {}", oxygen_generator_rating * co2_scrubber_rating);
}

#[test]
fn test_str_to_bits() {
    assert_eq!(from_str_bits("01101"), vec![0, 1, 1, 0, 1])
}
