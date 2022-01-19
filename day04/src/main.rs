use std::fs;
use std::collections::HashMap;
fn main() {
    part1();
    part2();
}
fn part1() {
    let mut total: i32 = 0;
    let input_file = fs::read_to_string("./input").expect("fail");
    let lines: Vec<&str> = input_file.split("\n").collect();
    for line in lines {
        let line_parts: Vec<&str> = line.split("[").collect();
        let input = line_parts[0];
        let checksum = line_parts[1].replace("]", "");
        let mut letters_and_number: Vec<&str> = input.split("-").collect();
        let number = letters_and_number[letters_and_number.len() - 1];
        letters_and_number.remove(letters_and_number.len() - 1);
        let letters = letters_and_number.join("");
        let mut letterMap: HashMap<char, i32> = HashMap::new();
        for letter in letters.chars() {
            if !letterMap.contains_key(&letter) {
                letterMap.insert(letter, 1);
            } else {
                let v: i32 = *letterMap.get(&letter).unwrap();
                letterMap.insert(letter, v+1);
            }
        }
        let mut letter_list: Vec<(&char, &i32)> = letterMap.iter().collect();
        letter_list.sort_by(|a, b| {
            if b.1 == a.1 {
                return a.0.cmp(b.0);
            }
            return b.1.cmp(a.1);   
        });
        
        let mut checksum_list: Vec<&char> = Vec::new();
        for (index, item) in letter_list {
                checksum_list.push(index);
                if checksum_list.len() == 5 {
                    break;
                }
        }
        let calc_checksum: String = checksum_list.into_iter().collect();
        if calc_checksum == checksum {
            total += number.parse::<i32>().unwrap();
        }
    }

    println!("Solution is {}", total);
}

fn part2() {
    let mut total: i32 = 0;
    let input_file = fs::read_to_string("./input").expect("fail");
    let lines: Vec<&str> = input_file.split("\n").collect();
    for line in lines {
        let line_parts: Vec<&str> = line.split("[").collect();
        let input = line_parts[0];
        let checksum = line_parts[1].replace("]", "");
        let mut letters_and_number: Vec<&str> = input.split("-").collect();
        let number = letters_and_number[letters_and_number.len() - 1];
        letters_and_number.remove(letters_and_number.len() - 1);
        let letters = letters_and_number.join("");
        let mut letterMap: HashMap<char, i32> = HashMap::new();
        for letter in letters.chars() {
            if !letterMap.contains_key(&letter) {
                letterMap.insert(letter, 1);
            } else {
                let v: i32 = *letterMap.get(&letter).unwrap();
                letterMap.insert(letter, v+1);
            }
        }
        let mut letter_list: Vec<(&char, &i32)> = letterMap.iter().collect();
        letter_list.sort_by(|a, b| {
            if b.1 == a.1 {
                return a.0.cmp(b.0);
            }
            return b.1.cmp(a.1);   
        });
        
        let mut checksum_list: Vec<&char> = Vec::new();
        for (index, item) in letter_list {
                checksum_list.push(index);
                if checksum_list.len() == 5 {
                    break;
                }
        }
        let calc_checksum: String = checksum_list.into_iter().collect();
        if calc_checksum == checksum {
            let mut new_words: Vec<String> = Vec::new();
            for word in letters_and_number {
                let mut new_word: Vec<char> = Vec::new();
                for c in word.chars() {                    
                    let alpha = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
                    let jump: usize = number.parse::<usize>().unwrap() % 26;
                    for i in 0..alpha.len() {
                        if alpha[i] == c {
                            let mut new_index = i + jump;
                            if new_index > 25 {
                                new_index = new_index - 26;
                            }
                            new_word.push(alpha[new_index]);
                        }
                    }
                }
                new_words.push(new_word.into_iter().collect());
            }

            println!("{} -> {}", new_words.join(" "), number);
        }
    }
}
