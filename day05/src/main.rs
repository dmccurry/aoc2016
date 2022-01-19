use md5;
fn main() {
    part2();
}

fn part1() {
    let code = "ffykfhsq";
    let mut index: usize = 0;
    let mut password: Vec<char> = Vec::new();
    loop {
        let digest = md5::compute(code.to_owned() + &index.to_string());
        let s = format!("{:x}", digest);
        if s.starts_with("00000") {
            println!("{}", index);
            password.push(s.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }
        index += 1;
    }
    let final_password: String = password.into_iter().collect();
    println!("{}", final_password);
}

fn part2() {
    let code = "ffykfhsq";
    let mut index: usize = 0;
    let mut num_set: usize = 0;
    let mut password: [char; 8] = ['\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'];
    loop {
        let digest = md5::compute(code.to_owned() + &index.to_string());
        let s = format!("{:x}", digest);
        if s.starts_with("00000") {
            println!("{}", index);
            let position_c = s.chars().nth(5).unwrap();
            
            if position_c == '0' || position_c == '1' || position_c == '2' || position_c == '3' || position_c == '4' || position_c == '5' || position_c == '6' || position_c == '7' {
                let position: usize = position_c.to_digit(10).unwrap().try_into().unwrap();
                let character = s.chars().nth(6).unwrap();

                if password[position] == '\0' {
                    password[position] = character;
                    num_set += 1;
                }
            }
            

            if num_set == 8 {
                break;
            }
        }
        index += 1;
    }
    let final_password: String = password.into_iter().collect();
    println!("{}", final_password);
}
