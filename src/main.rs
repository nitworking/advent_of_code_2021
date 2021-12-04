use std::fs;

pub fn count_increasing(depth_measurements: &[i32]) -> i32 {
    use std::i32::MAX;

    let mut bigger_than_previous: i32 = 0;

    let mut previous: i32 = MAX;
    for measurement in depth_measurements.iter() {
        if measurement > &previous {
            bigger_than_previous += 1;
        }
        previous = measurement.clone();
    }

    return bigger_than_previous;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_exercise() {
        let depth_measurements = [
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        assert_eq!(7, count_increasing(&depth_measurements));
    }
}


fn main() {
    let depth_measurements: Vec<i32> = fs::read_to_string("src/inputs/day_1_input.txt")
        .expect("Failed to read file")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let result = count_increasing(&depth_measurements);

    println!("Count: {}", result)
}
