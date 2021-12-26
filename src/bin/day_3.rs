use std::{fs};

fn _get_bit_counts(diagnostics: &Vec<String>) -> Vec<usize> {
    let diagnostic_length = diagnostics[0].len(); //assuming all same length
    let mut counts: Vec<usize> = vec![0; diagnostic_length];

    for diagnostic in diagnostics {
        for (index, digit) in diagnostic.chars().enumerate() {
            counts[index] += digit.to_digit(10).unwrap() as usize
        }
    }

    return counts;
}

fn calculate_power_consumption(diagnostics: &Vec<String>) -> i32 {
    let counts = _get_bit_counts(diagnostics);

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

fn calculate_life_support_rating(diagnostics: &Vec<String>) -> usize {
    let mut transposed: Vec<Vec<u32>> = vec![Vec::<u32>::new(); diagnostics[0].len()];

    for reading in diagnostics {
        for (index, digit) in reading.chars().enumerate() {
            transposed[index].push(digit.to_digit(10).unwrap())
        }
    }

    let oxygen_generator_rating = usize::from_str_radix(&diagnostics[_calculate_oxygen_generator_rating_index(&transposed)], 2).unwrap();
    let co2_scrubber_rating = usize::from_str_radix(&diagnostics[_calculate_co2_scrubber_rating_index(&transposed)], 2).unwrap();
    return oxygen_generator_rating * co2_scrubber_rating;
}

fn _calculate_oxygen_generator_rating_index(diagnostics: &Vec<Vec<u32>>) -> usize {
    let mut in_consideration: Vec<usize> = (0..diagnostics[0].len()).collect();
    let mut column_index = 0;

    while in_consideration.len() > 1 {
        let threshold = ((in_consideration.len() as f32) / 2.0).ceil() as u32;
        let mut sum = 0;
        
        for row_index in &in_consideration {
            sum += diagnostics[column_index][*row_index]
        }
        
        let most_common_bit = (sum >= threshold) as u32;
        let mut filtered: Vec<usize> = Vec::new();
        for row_index in &in_consideration {
            if diagnostics[column_index][*row_index] == most_common_bit {
                filtered.push(*row_index);
            }
        }

        in_consideration = filtered;
        column_index += 1;
    }

    return in_consideration[0];
}

fn _calculate_co2_scrubber_rating_index(diagnostics: &Vec<Vec<u32>>) -> usize {
    let mut in_consideration: Vec<usize> = (0..diagnostics[0].len()).collect();
    let mut column_index = 0;

    while in_consideration.len() > 1 {
        let threshold = ((in_consideration.len() as f32) / 2.0).ceil() as u32;
        let mut sum = 0;
        
        for row_index in &in_consideration {
            sum += diagnostics[column_index][*row_index]
        }
        
        let least_common_bit = (sum < threshold) as u32;
        let mut filtered: Vec<usize> = Vec::new();
        for row_index in &in_consideration {
            if diagnostics[column_index][*row_index] == least_common_bit {
                filtered.push(*row_index);
            }
        }

        in_consideration = filtered;
        column_index += 1;
    }

    return in_consideration[0];
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

    #[test]
    fn example_exercise_part_2() {
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

        let life_support_rating = calculate_life_support_rating(&diagnostics);

        assert_eq!(230, life_support_rating);
    }
}


fn main() {
    let diagnostics: Vec<String> = fs::read_to_string("inputs/day_3.txt")
        .expect("Failed to read file")
        .lines()
        .map(|diagnostic| String::from(diagnostic))
        .collect();

    println!("Power consumption: {}", calculate_power_consumption(&diagnostics));
    println!("Life support rating: {}", calculate_life_support_rating(&diagnostics));
}
