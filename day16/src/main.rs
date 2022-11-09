fn main() {
    part1();
    part2();
}

fn part1() {
    let input: &str = "01111001100111011";
    let mut file: String = input.to_string();
    let length = 272;

    while file.len() < length {
        file = dragon(file);
    }
    file = file[..length].to_string();
    println!("{}", checksum(file))
}

fn part2() {
    let input: &str = "01111001100111011";
    let mut file: String = input.to_string();
    let length = 35651584;

    while file.len() < length {
        file = dragon(file);
    }
    file = file[..length].to_string();
    println!("{}", checksum(file))
}

fn dragon(input: String) -> String {
    let mut a: String = input.clone().to_owned();
    let b_chars: Vec<char> = input.clone().chars().rev().collect();
    a.push_str("0");
    for c in b_chars {
        if c == '0' {
            a.push_str("1");
        } else {
            a.push_str("0");
        }
    }
    return a;
}

fn checksum(input: String) -> String {
    let mut sum = "".to_string().to_owned();
    let file_chars: Vec<char> = input.chars().collect();

    let mut i: usize = 0;
    while i < file_chars.len() {
        if (file_chars[i] == file_chars[i + 1]) {
            sum.push_str("1");
        } else {
            sum.push_str("0")
        }
        i = i + 2;
    }

    if (sum.len() % 2 == 0) {
        return checksum(sum);
    } else {
        return sum;
    }
}
