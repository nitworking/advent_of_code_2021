use std::{fs};

pub fn calculate_destination(course: &Vec<(String, i32)>) -> (i32, i32) {
    let mut hpos = 0;
    let mut depth = 0;
    
    for (direction, units) in course {
        match direction.as_str() {
            "forward" => hpos += units,
            "up" => depth -= units,
            "down" => depth += units,
            _ => eprintln!("Unknown direction {}", direction)
        }
    }

    return (hpos, depth);
}

pub fn calculate_destination_2(course: &Vec<(String, i32)>) -> (i32, i32) {
    let mut hpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for (direction, units) in course {
        match direction.as_str() {
            "forward" => {
                hpos += units;
                depth += aim * units;
            },
            "up" => aim -= units,
            "down" => aim += units,
            _ => eprintln!("Unknown direction {}", direction)
        }
    }

    return (hpos, depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_exercise_part_1() {
        let course = vec!(
            (String::from("forward"), 5),
            (String::from("down"), 5),
            (String::from("forward"), 8),
            (String::from("up"), 3),
            (String::from("down"), 8),
            (String::from("forward"), 2),
        );

        let location = calculate_destination(&course);

        assert_eq!(150, location.0 * location.1);
    }

    #[test]
    fn example_exercise_part_2() {
        let course = vec!(
            (String::from("forward"), 5),
            (String::from("down"), 5),
            (String::from("forward"), 8),
            (String::from("up"), 3),
            (String::from("down"), 8),
            (String::from("forward"), 2),
        );

        let location = calculate_destination_2(&course);

        assert_eq!(900, location.0 * location.1);
    }
}


fn main() {
    let course: Vec<(String, i32)> = fs::read_to_string("src/inputs/day_2.txt")
        .expect("Failed to read file")
        .lines()
        .map(|command| -> (String, i32) { 
            let (direction, units) = command.split_once(" ").unwrap();
            let units = units.parse::<i32>().unwrap();
            (direction.to_string(), units)
        })
        .collect();

    let (hpos, depth) = calculate_destination(&course);
    println!("Multiplied: {}", hpos*depth);

    let (hpos, depth) = calculate_destination_2(&course);
    println!("Multiplied 2: {}", hpos*depth);
}
