use std::collections::{HashMap, HashSet};
use regex::Regex;

use crate::utils;

// #[derive(Debug)]
// enum PassportKeys {
//     BYr, // (Birth Year)
//     IYr, // (Issue Year)
//     EYr, // (Expiration Year)
//     Hgt, // (Height)
//     HCl, // (Hair Color)
//     ECl, // (Eye Color)
//     PId, // (Passport ID)
//     CId, // (Country ID)
// }

// impl From<String> for PassportKeys {
//     fn from(key_val: String) -> Self {
//         let key_val_vec: Vec<String> = key_val
//             .split(":")
//             .map(|x| String::from_str(x).unwrap())
//             .collect();

//         if key_val_vec.len() < 2 {
//             panic!("Wrong key conversion: {:?}", key_val)
//         }

//         return match key_val_vec[0].as_str() {
//             "byr" => PassportKeys::BYr,
//             "iyr" => PassportKeys::IYr,
//             "eyr" => PassportKeys::EYr,
//             "hgt" => PassportKeys::Hgt,
//             "hcl" => PassportKeys::HCl,
//             "ecl" => PassportKeys::ECl,
//             "pid" => PassportKeys::PId,
//             "cid" => PassportKeys::CId,
//             _ => panic!("Wrong key conversion: {:?}", key_val),
//         };
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
struct Passport {
    data_map: HashMap<String, String>,
    missing_keys: HashSet<String>,
}

impl Passport {
    fn new() -> Self {
        let missing_def: HashSet<String> = [
            "byr".to_string(),
            "iyr".to_string(),
            "eyr".to_string(),
            "hgt".to_string(),
            "hcl".to_string(),
            "ecl".to_string(),
            "pid".to_string(),
            "cid".to_string(),
        ]
        .into();

        Passport {
            data_map: HashMap::<String, String>::new(),
            missing_keys: missing_def,
        }
    }

    fn insert(&mut self, key: String, value: String) {
        if self.missing_keys.contains(&key) {
            self.missing_keys.remove(&key);
            self.data_map.insert(key, value);
        } else {
            panic!("Same keys in the same passport: {:?}", key);
        }
    }
}

fn is_valid(pass: &Passport) -> bool {
    let mut checks = 0;

    for (key, val) in pass.data_map.clone() {
        if key.eq("byr") { // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            let parse_val: usize = match val.parse::<usize>() {
                Ok(res) => res,
                Err(_) => continue,
            };
            if parse_val >= 1920 && parse_val <= 2002 {
                checks += 1;
                continue;
            }
        }
        if key.eq("iyr") { // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            let parse_val: usize = match val.parse::<usize>() {
                Ok(res) => res,
                Err(_) => continue,
            };
            if parse_val >= 2010 && parse_val <= 2020 {
                checks += 1;
                continue;
            }
        }
        if key.eq("eyr") { // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            let parse_val: usize = match val.parse::<usize>() {
                Ok(res) => res,
                Err(_) => continue,
            };
            if parse_val >= 2020 && parse_val <= 2030 {
                checks += 1;
                continue;
            }
        }
        if key.eq("hgt") { // hgt (Height) - a number followed by either cm or in: 
            if val.contains("cm") { // If cm, the number must be at least 150 and at most 193.
                let parse_val: usize = val.replace("cm", "").parse::<usize>().unwrap();
                if parse_val >= 150 && parse_val <= 193 {
                    checks += 1;
                    continue;
                }
            }
            if val.contains("in") { // If in, the number must be at least 59 and at most 76.
                let parse_val: usize = val.replace("in", "").parse::<usize>().unwrap();
                if parse_val >= 59 && parse_val <= 76 {
                    checks += 1;
                    continue;
                }
            }
        }
        if key.eq("hcl") { // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            let re = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
            if re.is_match(val.as_str()){
                checks += 1;
                continue;
            }
        }
        if key.eq("ecl") { // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if valid_colors.contains(&val.as_str()){
                checks += 1;
                continue;         
            }
        }
        if key.eq("pid") { // pid (Passport ID) - a nine-digit number, including leading zeroes.
            let re = Regex::new(r"^\d{9}$").unwrap();
            if re.is_match(val.as_str()) {
                checks += 1;
                continue;
            }
        }
    }

    return checks == 7;
}

#[allow(dead_code)]
pub fn run_part_1() -> Result<usize, String> {
    let raw_input = utils::read_input("src/day_4/input").unwrap();

    let mut passports: Vec<Passport> = Vec::new();

    let mut temp_pass = Passport::new();

    for line in raw_input {
        let tokenized_line: Vec<&str> = line.split(" ").collect();

        if tokenized_line.len() == 1 && tokenized_line.contains(&"") {
            passports.push(temp_pass.clone());
            temp_pass = Passport::new();
            continue;
        }

        for item in tokenized_line {
            let (key, val) = {
                let mut temp_pair = item.split(":");

                let temp_key = temp_pair.next();
                let temp_val = temp_pair.next();

                (temp_key.unwrap().to_string(), temp_val.unwrap().to_string())
            };

            temp_pass.insert(key, val);
        }
    }
    passports.push(temp_pass.clone());

    let mut count = 0;

    for pass in passports {
        if is_valid(&pass) {
            count += 1;
        }
    }

    return Ok(count);
}
