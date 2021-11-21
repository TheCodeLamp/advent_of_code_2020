use std::collections::HashSet;

use crate::utils;

#[allow(dead_code)]
pub fn run_part_1() -> Result<usize, String> {
    let raw_input = utils::read_input("src/day_6/input").unwrap();

    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut group: HashSet<char> = HashSet::new();

    for line in raw_input {

        for char in line.chars(){
            group.insert(char);
        }

        if line.len() == 0{
            groups.push(group.clone());
            group = HashSet::new();
        }
    }

    groups.push(group);

    let mut count = 0;

    for group in groups{
        count += group.len();
    }

    return Ok(count);
}
