use crate::utils;

fn next_position_right(length: usize, start: usize, jump: usize) -> usize {
    let mut temp = start;
    temp += jump;
    if temp >= length {
        temp -= length;
    }
    temp
}

fn number_of_trees_traversal_tup(dir: (usize, usize)) -> usize {
    number_of_trees_traversal(dir.0, dir.1)
}

fn number_of_trees_traversal(right: usize, down: usize) -> usize {
    let raw_input = utils::read_input("src/day_3/input").unwrap();

    let mut number_of_trees: usize = 0;
    let mut current_pos: usize = 0;

    let mut down_count = 1;

    for line in raw_input {
        down_count -= 1;
        if down_count != 0 {
            continue;
        } else {
            down_count = down;
        }

        if {
            match line.chars().skip(current_pos).next() {
                Some(res) => res,
                None => panic!("{} {}", line.len(), current_pos),
            }
        } == '#'
        {
            number_of_trees += 1;
        }
        current_pos = next_position_right(line.len(), current_pos, right)
    }

    number_of_trees
}

#[allow(dead_code)]
pub fn run_part_1() -> Result<usize, String> {
    return Ok(number_of_trees_traversal(3, 1));
}

#[allow(dead_code)]
pub fn run_part_2() -> Result<usize, String> {
    let traversal: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut out = 1;

    for trav in traversal {
        out *= number_of_trees_traversal_tup(trav);
        println!("{:?}", number_of_trees_traversal_tup(trav));
    }

    Ok(out)
}
