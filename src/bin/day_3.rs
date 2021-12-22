use std::{fs};

pub fn calculate_power_consumption(diagnostics: &Vec<String>) -> i32 {
    let diagnostic_length = diagnostics[0].len(); //assuming all same length
    let mut counts: Vec<usize> = vec![0; diagnostic_length];

    for diagnostic in diagnostics {
        for (index, digit) in diagnostic.chars().enumerate() {
            counts[index] += digit.to_digit(10).unwrap() as usize
        }
    }

    let threshold: usize = diagnostics.len()/2;
    let mut gamma = 0b0;
    let mut epsilon = 0b0;

    for count in counts {
        if count > threshold {
            gamma += 0b1;
        } else {
            epsilon += 0b1;
        }

        gamma = gamma << 1;
        epsilon = epsilon << 1;
    }

    //FIXME: shifted once too many so shift back. This one is quite the hack!
    gamma = gamma >> 1;
    epsilon = epsilon >> 1;

    return gamma * epsilon;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_exercise_part_1() {
        let diagnostics: Vec<String> = vec!(
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        );

        let power_consumption = calculate_power_consumption(&diagnostics);

        assert_eq!(198, power_consumption);
    }
}


fn main() {
    let diagnostics: Vec<String> = fs::read_to_string("inputs/day_3.txt")
        .expect("Failed to read file")
        .lines()
        .map(|diagnostic| String::from(diagnostic))
        .collect();

    println!("Power consumption: {}", calculate_power_consumption(&diagnostics))
}
