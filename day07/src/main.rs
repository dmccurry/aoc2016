use std::fs;
fn main() {
    part1();
    part2();
}
fn part1() {
    let input_file = fs::read_to_string("./input").expect("dead");

    let lines = input_file.split("\n");
    let mut total = 0;
    for line in lines {
        let mut is_invalid = false;
        let mut found_string = false;
        let mut found_invalid = false;
        for (i, c) in line.chars().enumerate() {
            if c == '[' {
                is_invalid = true
            } else if c == ']' {
                is_invalid = false;
            } else {
                if i < line.len() - 3 {
                    let a = c;
                    let b = line.chars().nth(i+1).unwrap();
                    let c = line.chars().nth(i+2).unwrap();
                    let d = line.chars().nth(i+3).unwrap();

                    if a != b && a == d && b == c {
                        found_string = true;
                        found_invalid = found_invalid | is_invalid;
                    }
                }
            }
        }
        if found_string && !found_invalid {
            total += 1;
        }
    }
    println!("{}", total);
}

fn part2() {
    let input_file = fs::read_to_string("./input").expect("dead");

    let lines = input_file.split("\n");
    let mut total = 0;
    for line in lines {
        let mut is_invalid = false;
        let mut is_match = false;
        for (i, c) in line.chars().enumerate() {
            if c == '[' {
                is_invalid = true
            } else if c == ']' {
                is_invalid = false;
            } else {
                if i < line.len() - 2 {
                    let a = c;
                    let b = line.chars().nth(i+1).unwrap();
                    let c = line.chars().nth(i+2).unwrap();

                    if a != b && a == c && !is_invalid {
                        let mut in_brackets = false;
                        for (j, d) in line.chars().enumerate() {
                            if d == '[' {
                                in_brackets = true;
                            } else if d == ']' {
                                in_brackets = false;
                            } else {
                                if in_brackets && j < line.len() - 2 && d == b {
                                    if line.chars().nth(j+1).unwrap() == a && line.chars().nth(j+2).unwrap() == d {
                                        is_match = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if is_match {
            total += 1;
        }
    }
    println!("{}", total);
}
