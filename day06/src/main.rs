use std::fs;
use std::collections::HashMap;
fn main() {
    part1();
    part2();
}
fn part1() {
    let input_file = fs::read_to_string("./input").expect("dead");
    let input_lines: Vec<&str> = input_file.split("\n").collect();
    let mut lines: Vec<String> = Vec::new();
    let mut line_length: usize = 0;
    let mut output: Vec<char> = Vec::new();
    for line in input_lines {
        lines.push(line.to_string());
        line_length = line.len();
    }

    for i in 0..line_length {
        let mut letter_map: HashMap<char, i32> = HashMap::new();

        for line in lines.iter() {
            let ch = line.chars().nth(i).unwrap();
            if !letter_map.contains_key(&ch) {
                letter_map.insert(ch, 1);
            } else {
                let v: i32 = *letter_map.get(&ch).unwrap();
                letter_map.insert(ch, v+1);
            }
        }
        let mut letter_list: Vec<(&char, &i32)> = letter_map.iter().collect();
        letter_list.sort_by(|a, b| b.1.cmp(a.1));
        let item = letter_list.iter().next().unwrap();
        output.push(*item.0);

    }

    let final_output: String = output.into_iter().collect();
    println!("{}", final_output);
    
}

fn part2() {
    let input_file = fs::read_to_string("./input").expect("dead");
    let input_lines: Vec<&str> = input_file.split("\n").collect();
    let mut lines: Vec<String> = Vec::new();
    let mut line_length: usize = 0;
    let mut output: Vec<char> = Vec::new();
    for line in input_lines {
        lines.push(line.to_string());
        line_length = line.len();
    }

    for i in 0..line_length {
        let mut letter_map: HashMap<char, i32> = HashMap::new();

        for line in lines.iter() {
            let ch = line.chars().nth(i).unwrap();
            if !letter_map.contains_key(&ch) {
                letter_map.insert(ch, 1);
            } else {
                let v: i32 = *letter_map.get(&ch).unwrap();
                letter_map.insert(ch, v+1);
            }
        }
        let mut letter_list: Vec<(&char, &i32)> = letter_map.iter().collect();
        letter_list.sort_by(|a, b| a.1.cmp(b.1));
        let item = letter_list.iter().next().unwrap();
        output.push(*item.0);

    }

    let final_output: String = output.into_iter().collect();
    println!("{}", final_output);
    
}
