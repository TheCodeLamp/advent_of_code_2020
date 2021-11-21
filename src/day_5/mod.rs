
use crate::utils;

fn translate_seat(pass: String) -> (usize, usize){
    assert!(pass.len() == 10);

    let mut row_range: Vec<usize> = (0..127).collect();
    let mut columns_range: Vec<usize> = (0..7).collect();

    for char in pass.chars(){
        
        let row_middle = row_range.len()/2;

        if char == 'F' {
            row_range = row_range.split_off(row_middle);
        } else if char == 'B' {
            let _ = row_range.split_off(row_middle);
        }

        let columns_middle = columns_range.len()/2;

        if char == 'R' {
            columns_range = columns_range.split_off(columns_middle);
        } else if char == 'L' {
            let _ = columns_range.split_off(columns_middle);
        }
    }

    assert_eq!(row_range.len(), 1);
    assert_eq!(columns_range.len(), 1);

    return (0,0);

}


pub fn run_part_1() -> Result<usize, String>{
    let raw_input = utils::read_input("src/day_5/input").unwrap();

    

    return Err("Not finished".to_string())
}