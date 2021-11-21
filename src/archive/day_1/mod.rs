use std::collections::BTreeSet;

use crate::utils;

fn get_numbers() -> (Vec<usize>, BTreeSet<usize>) {
    let input = utils::read_input("src/day_1/input").unwrap();

    let mut numbers: Vec<usize> = Vec::new();
    let mut number_set: BTreeSet<usize> = BTreeSet::new();

    for line in input {
        let temp = line.parse::<usize>().unwrap();
        numbers.push(temp);
        number_set.insert(temp);
    }

    return (numbers, number_set);
}

#[allow(dead_code)]
pub fn run_part_1() -> Result<usize, String> {
    let (numbers, number_set) = get_numbers();

    for number in numbers {
        let current_number = 2020 - number;
        if number_set.contains(&current_number) {
            return Ok(current_number * number);
        }
    }

    Err("Couldn't find desired number.".to_string())
}

#[allow(dead_code)]
pub fn run_part_2() -> Result<usize, String> {
    let (numbers, number_set) = get_numbers();

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            let current_number: isize = 2020 - (numbers[i] as isize + numbers[j] as isize);
            if current_number < 0 {
                continue;
            };

            if number_set.contains(&(current_number as usize)) {
                return Ok(current_number as usize * numbers[i] * numbers[j]);
            }
        }
    }

    Err("Couldn't find desired number.".to_string())
}
