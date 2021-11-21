use crate::utils;

fn get_lists() -> (Vec<String>, Vec<((usize, usize), char)>) {
    let raw_input = utils::read_input("src/day_2/input").unwrap();

    let mut passwords: Vec<String> = Vec::new();
    let mut raw_password_policies: Vec<String> = Vec::new();

    for pair in raw_input {
        let mut temp_pair = pair.split(": ");
        raw_password_policies.push(temp_pair.next().unwrap().to_string());
        passwords.push(temp_pair.next().unwrap().to_string());
    }

    let mut password_policies: Vec<((usize, usize), char)> = Vec::new();
    for policy in raw_password_policies.iter() {
        let mut policy_split = policy.split(" ");
        let range: (usize, usize) = {
            let mut temp = policy_split.next().unwrap().split("-");
            (
                temp.next().unwrap().parse().unwrap(),
                temp.next().unwrap().parse().unwrap(),
            )
        };
        let letter: char = {
            let temp = policy_split.next().unwrap();
            temp.parse().unwrap()
        };

        password_policies.push((range, letter))
    }

    (passwords, password_policies)
}

#[allow(dead_code)]
pub fn run_part_1() -> Result<usize, String> {
    let (passwords, password_policies) = get_lists();

    let mut number_of_passwords: usize = 0;

    for (i, policy) in password_policies.iter().enumerate() {
        let count = passwords[i].matches(policy.1).count();
        if ((policy.0 .0)..(policy.0 .1 + 1)).contains(&count) {
            number_of_passwords += 1;
        }
    }

    if number_of_passwords == 0 {
        return Err("Number of passwords could not be found.".to_string());
    }

    return Ok(number_of_passwords);
}

#[allow(dead_code)]
pub fn run_part_2() -> Result<usize, String> {
    let (passwords, password_policies) = get_lists();

    let mut number_of_passwords: usize = 0;

    for (i, policy) in password_policies.iter().enumerate() {
        let mut temp_count = 0;
        if passwords[i].chars().skip(policy.0 .0 - 1).next().unwrap() == policy.1 {
            temp_count += 1;
        }
        if passwords[i].chars().skip(policy.0 .1 - 1).next().unwrap() == policy.1 {
            temp_count += 1;
        }
        if temp_count == 1 {
            number_of_passwords += 1;
        }
    }

    if number_of_passwords == 0 {
        return Err("Number of passwords could not be found.".to_string());
    }

    return Ok(number_of_passwords);
}
