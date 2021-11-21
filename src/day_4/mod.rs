use std::{
    collections::{HashMap, HashSet},
    panic,
    str::FromStr,
};

use crate::utils;

#[derive(Debug)]
enum PassportKeys {
    BYr, // (Birth Year)
    IYr, // (Issue Year)
    EYr, // (Expiration Year)
    Hgt, // (Height)
    HCl, // (Hair Color)
    ECl, // (Eye Color)
    PId, // (Passport ID)
    CId, // (Country ID)
}

impl From<String> for PassportKeys {
    fn from(key_val: String) -> Self {
        let key_val_vec: Vec<String> = key_val
            .split(":")
            .map(|x| String::from_str(x).unwrap())
            .collect();

        if key_val_vec.len() < 2 {
            panic!("Wrong key conversion: {:?}", key_val)
        }

        return match key_val_vec[0].as_str() {
            "byr" => PassportKeys::BYr,
            "iyr" => PassportKeys::IYr,
            "eyr" => PassportKeys::EYr,
            "hgt" => PassportKeys::Hgt,
            "hcl" => PassportKeys::HCl,
            "ecl" => PassportKeys::ECl,
            "pid" => PassportKeys::PId,
            "cid" => PassportKeys::CId,
            _ => panic!("Wrong key conversion: {:?}", key_val),
        };
    }
}

#[derive(Debug, Clone)]
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

    // fn 7
}

fn is_valid(pass: &Passport) -> bool {
    if pass.missing_keys.len() > 0 {
        if pass.missing_keys.len() == 1 && pass.missing_keys.contains(&"cid".to_string()) {
            return true;
        }
    } else {
        return true;
    }

    return false;
}

#[allow(dead_code)]
pub fn run_part_1() -> Result<usize, String> {
    let raw_input = utils::read_input("src/day_4/input").unwrap();

    let mut passports: Vec<Passport> = Vec::new();

    let mut temp_pass = Passport::new();

    for line in raw_input {
        //println!("Raw line: {:?}", line);
        let tokenized_line: Vec<&str> = line.split(" ").collect();
        //println!("Token Line: {:?}; len: {:?}", tokenized_line, tokenized_line.len());

        if tokenized_line.len() == 1 && tokenized_line.contains(&"") {
            passports.push(temp_pass.clone());
            temp_pass = Passport::new();
            continue;
        }

        for item in tokenized_line {
            let (key, val) = {
                let mut temp_pair= item.split(":");
                
                let temp_key = temp_pair.next();
                let temp_val = temp_pair.next();

                // println!("(key:val) : ({:?}:{:?})", temp_key, temp_val);

                (
                    temp_key.unwrap().to_string(),
                    temp_val.unwrap().to_string(),
                )
            };

            temp_pass.insert(key, val);
        }
    }

    let mut count = 0;

    for pass in passports{
        if is_valid(&pass){
            count += 1;
        }else{
            //println!("{:?}", pass);
        }
    }

    return Ok(count);

    //return Err("Not finished.".to_string());
}
