use std::fs;
fn main() {
    part1();
    part2();
}

fn part1() {
    let input_file = fs::read_to_string("./input").expect("fail");
    let mut valid: Vec<String> = Vec::new();
    let lines = input_file.split("\n");

    for line in lines {
        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("error"))
            .collect();
        
        nums.sort();
        if nums[2] < (nums[0] + nums[1]) {
            valid.push(line.to_string());
        }

    }

    println!("{}", valid.len());
}

fn part2() {
    let input_file = fs::read_to_string("./input").expect("fail");
    let mut valid = 0;
    let lines: Vec<&str> = input_file.split("\n").collect();
    
    let mut i = 0;
    while i < lines.len() {
        let nums: Vec<i32> = lines[i]
            .split_whitespace()
            .map(|s| s.parse().expect("error"))
            .collect();
        let nums2: Vec<i32> = lines[i + 1]
            .split_whitespace()
            .map(|s| s.parse().expect("error"))
            .collect();
        let nums3: Vec<i32> = lines[i + 2]
            .split_whitespace()
            .map(|s| s.parse().expect("error"))
            .collect();
        
        let mut n = [nums[0], nums2[0], nums3[0]];
        n.sort();
        if n[2] < (n[0] + n[1]) {
            valid += 1;
        }

        n = [nums[1], nums2[1], nums3[1]];
        n.sort();
        if n[2] < (n[0] + n[1]) {
            valid += 1;
        }
        n = [nums[2], nums2[2], nums3[2]];
        n.sort();
        if n[2] < (n[0] + n[1]) {
            valid += 1;
        }

        i = i + 3;
    }

    println!("{}", valid);
}
