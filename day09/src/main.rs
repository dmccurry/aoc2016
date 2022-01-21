use std::fs;
fn main() {
    // part1();
    part2();
}

fn part1() {
    let input_file = fs::read_to_string("./input");
    let mut output: Vec<char> = Vec::new();
    let mut current_index: usize = 0;
    let chars: Vec<char> = input_file.unwrap().chars().collect();
    
    while current_index < chars.len() {
        let current_char = chars[current_index];
        if current_char == '(' {
            current_index += 1;
            let mut next_char = chars[current_index]; 
            let mut numbers: String = String::from("");
            while next_char != ')' {
                numbers.push(next_char);
                current_index += 1;
                next_char = chars[current_index];
            }
            let mut num_split = numbers.split("x");
            let num_chars: usize = num_split.next().unwrap().parse().unwrap();
            let num_repeats: usize = num_split.next().unwrap().parse().unwrap();
            let mut char_count: usize = 0;
            let mut chars_to_repeat: Vec<char> = Vec::new();
            while char_count < num_chars {
                current_index += 1;
                chars_to_repeat.push(chars[current_index]);
                output.push(chars[current_index]);
                char_count += 1;
            }
            
            for i in 1..num_repeats {
                for j in 0..chars_to_repeat.len() {
                    output.push(chars_to_repeat[j]);
                }
            }
            current_index += 1;
            
        } else {
            output.push(current_char);
            current_index += 1;
        }
    }
    println!("{}", output.len());
}

fn part2() {
    let input_file = fs::read_to_string("./input");
    let mut output: Vec<usize> = Vec::new();
    let mut current_index: usize = 0;
    let chars: Vec<char> = input_file.unwrap().chars().collect();
    
    for i in 0..chars.len() {
        output.push(1);
    }
    let mut weight: usize = 0;
    while current_index < chars.len() {
        let current_char = chars[current_index];
        if current_char == '(' {
            current_index += 1;
            let mut next_char = chars[current_index]; 
            let mut numbers: String = String::from("");
            while next_char != ')' {
                numbers.push(next_char);
                current_index += 1;
                next_char = chars[current_index];
            }
            let mut num_split = numbers.split("x");
            let num_chars: usize = num_split.next().unwrap().parse().unwrap();
            let num_repeats: usize = num_split.next().unwrap().parse().unwrap();
            let mut char_count: usize = 0;
            let mut chars_to_repeat: Vec<char> = Vec::new();
            while char_count < num_chars {
                let my_index = current_index + 1 + char_count;
                output[my_index] = output[my_index] * num_repeats;
                char_count += 1;
            }
            current_index += 1;
            
        } else {
            weight += output[current_index];
            current_index += 1;
        }
    }
    println!("{}", weight);
}
