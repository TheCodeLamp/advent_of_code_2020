use std::collections::BTreeSet;

use crate::utils;

fn translate_seat(pass: &String) -> (usize, usize) {
    assert!(pass.len() == 10);

    let mut row_range: Vec<usize> = (0..128).collect();
    let mut columns_range: Vec<usize> = (0..8).collect();

    for char in pass.chars() {
        let row_middle = row_range.len() / 2;

        if char == 'B' {
            row_range = row_range.split_off(row_middle);
        } else if char == 'F' {
            let _ = row_range.split_off(row_middle);
        }

        let columns_middle = columns_range.len() / 2;

        if char == 'R' {
            columns_range = columns_range.split_off(columns_middle);
        } else if char == 'L' {
            let _ = columns_range.split_off(columns_middle);
        }
    }

    assert_eq!(row_range.len(), 1, "Row fail");
    assert_eq!(columns_range.len(), 1, "Column fail");

    return (row_range[0], columns_range[0]);
}

#[allow(dead_code)]
pub fn run_part_1() -> Result<usize, String> {
    let raw_input = utils::read_input("src/day_5/input").unwrap();

    let mut biggest = 0;

    for line in raw_input {
        let (row, column) = translate_seat(&line);

        let seat_id = row * 8 + column;

        if seat_id > biggest {
            biggest = seat_id;
        }
    }

    return Ok(biggest);
}

#[allow(dead_code)]
pub fn run_part_2() -> Result<usize, String> {
    let raw_input = utils::read_input("src/day_5/input").unwrap();

    let mut seat_ids_set: BTreeSet<usize> = BTreeSet::new();

    for line in raw_input {
        let (row, column) = translate_seat(&line);
        seat_ids_set.insert(row * 8 + column);
    }

    let mut last: Option<usize> = None;

    for id in seat_ids_set {
        if last == None {
            last = Some(id);
            continue;
        }

        if id-1 != last.unwrap() {
            return Ok(id-1);
        }

        last = Some(id);
    }

    return Err("Didn't find seat.".to_string());
}
