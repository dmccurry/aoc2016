use std::fs;
use std::collections::HashMap;

fn main() {
    part1();

    part2();
}

fn part1() {
    let input_file = fs::read_to_string("./input").expect("fail");

    let lines: Vec<&str> = input_file.split("\n").collect();
    let mut registers: HashMap<&str, i32> = HashMap::new();
    registers.insert("a", 0);
    registers.insert("b", 0);
    registers.insert("c", 0);
    registers.insert("d", 0);
    let mut i: i32 = 0;

    while i < lines.len().try_into().unwrap() && i >= 0 {
        let index: usize = i.try_into().unwrap();
        let instruction = lines[index];
        let instruction_part: Vec<&str> = instruction.split(" ").collect();
        if instruction_part[0] == "cpy" {
            let dest = instruction_part[2];
            let src = instruction_part[1];

            let value = registers.get(&dest).unwrap();

            if src == "a" || src == "b" || src == "c" || src == "d" {
                let new_value = registers.get(&src).unwrap();
                registers.insert(dest, *new_value);
            } else {
                let new_value: i32 = src.parse().unwrap();
                registers.insert(dest, new_value);
            }
            i += 1;
        } else if instruction_part[0] == "inc" {
            let dest = instruction_part[1];
            let value = registers.get(&dest).unwrap();
            registers.insert(dest, value + 1);
            i += 1;
        } else if instruction_part[0] == "dec" {
            let dest = instruction_part[1];
            let value = registers.get(&dest).unwrap();
            registers.insert(dest, value - 1);
            i += 1;
        } else if instruction_part[0] == "jnz" {
            let dest = instruction_part[1];
            let amt: i32 = instruction_part[2].parse().unwrap();

            if dest == "a" || dest == "b" || dest == "c" || dest == "d" {
                let value = registers.get(&dest).unwrap();
                if *value != 0 {
                    i += amt;
                } else {
                    i += 1;
                }
            } else {
                let value: i32 = dest.parse().unwrap();
                if value != 0 {
                    i += amt;
                } else {
                    i += 1;
                }
            }
            
        }
    }

    println!("{:?}", registers);
}

fn part2() {
    let input_file = fs::read_to_string("./input").expect("fail");

    let lines: Vec<&str> = input_file.split("\n").collect();
    let mut registers: HashMap<&str, i32> = HashMap::new();
    registers.insert("a", 0);
    registers.insert("b", 0);
    registers.insert("c", 1);
    registers.insert("d", 0);
    let mut i: i32 = 0;

    while i < lines.len().try_into().unwrap() && i >= 0 {
        let index: usize = i.try_into().unwrap();
        let instruction = lines[index];
        let instruction_part: Vec<&str> = instruction.split(" ").collect();
        if instruction_part[0] == "cpy" {
            let dest = instruction_part[2];
            let src = instruction_part[1];

            let value = registers.get(&dest).unwrap();

            if src == "a" || src == "b" || src == "c" || src == "d" {
                let new_value = registers.get(&src).unwrap();
                registers.insert(dest, *new_value);
            } else {
                let new_value: i32 = src.parse().unwrap();
                registers.insert(dest, new_value);
            }
            i += 1;
        } else if instruction_part[0] == "inc" {
            let dest = instruction_part[1];
            let value = registers.get(&dest).unwrap();
            registers.insert(dest, value + 1);
            i += 1;
        } else if instruction_part[0] == "dec" {
            let dest = instruction_part[1];
            let value = registers.get(&dest).unwrap();
            registers.insert(dest, value - 1);
            i += 1;
        } else if instruction_part[0] == "jnz" {
            let dest = instruction_part[1];
            let amt: i32 = instruction_part[2].parse().unwrap();

            if dest == "a" || dest == "b" || dest == "c" || dest == "d" {
                let value = registers.get(&dest).unwrap();
                if *value != 0 {
                    i += amt;
                } else {
                    i += 1;
                }
            } else {
                let value: i32 = dest.parse().unwrap();
                if value != 0 {
                    i += amt;
                } else {
                    i += 1;
                }
            }
            
        }
    }

    println!("{:?}", registers);
}
