use std::{fs::File, io::{BufRead, BufReader}};

#[allow(dead_code)]
pub fn read_input(path: &str) -> Result<Vec<String>, String> {
    let file = match File::open(path){
        Ok(temp_ok) => temp_ok,
        Err(error) => return Err(error.to_string()),
    };
    let reader = BufReader::new(file);

    let mut output: Vec<String> = [].into();

    for line in reader.lines() {
        match line {
            Ok(input) => output.push(input),
            Err(error) => return Err(error.to_string()),
        };
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_file() {

        match super::read_input("src/utils/test.txt") {
            Ok(lines) => println!("Ok: {}", &lines[0]),
            Err(error) => println!("Error: {}", &error),
        }
    }
}
