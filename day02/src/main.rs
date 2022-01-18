use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let keypad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let input_file = fs::read_to_string("./input")
        .expect("fail");

    let dirs = input_file.split("\n");
    let mut current_x: i32 = 1;
    let mut current_y: i32 = 1;
    let mut code = "";

    let mut codes: Vec<String> = Vec::new();

    for dir in dirs {
        for c in dir.chars() {
            if c == 'R' {
                current_x += 1;
            } else if c == 'L' {
                current_x -= 1;
            } else if c == 'U' {
                current_y -= 1;
            } else if c == 'D' {
                current_y += 1;
            }
            if current_x < 0 {
                current_x = 0;
            }
            if current_y < 0 {
                current_y = 0;
            }
            if current_x > 2 {
                current_x = 2;
            }
            if current_y > 2 {
                current_y = 2;
            }
        }
        codes.push(keypad[current_y as usize][current_x as usize].to_string())
    }
    println!("Solution is {}", codes.join(""));
}

fn part2() {
    let keypad = [[' ', ' ', '1', ' ', ' '], [' ', '2', '3', '4', ' '], ['5', '6', '7', '8', '9'], [' ', 'A', 'B', 'C', ' '], [' ', ' ', 'D', ' ', ' ']];
    let input_file = fs::read_to_string("./input")
        .expect("fail");

    let dirs = input_file.split("\n");
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 2;
    let mut prev_x: i32 = 0;
    let mut prev_y: i32 = 2;
    let mut code = "";

    let mut codes: Vec<String> = Vec::new();

    for dir in dirs {
        for c in dir.chars() {
            prev_x = current_x;
            prev_y = current_y;
            if c == 'R' {
                current_x += 1;
                if current_x < 0 || current_x > 4 {
                    current_x = prev_x;
                } else if keypad[current_y as usize][current_x as usize] == ' ' {
                    current_x = prev_x;
                }
            } else if c == 'L' {
                current_x -= 1;
                if current_x < 0 || current_x > 4 {
                    current_x = prev_x;
                } else if keypad[current_y as usize][current_x as usize] == ' ' {
                    current_x = prev_x;
                }
            } else if c == 'U' {
                current_y -= 1;
                if current_y < 0 || current_y > 4 {
                    current_y = prev_y;
                } else if keypad[current_y as usize][current_x as usize]== ' ' {
                    current_y = prev_y;
                }
            } else if c == 'D' {
                current_y += 1;
                if current_y < 0 || current_y > 4 {
                    current_y = prev_y;
                } else if keypad[current_y as usize][current_x as usize] == ' ' {
                    current_y = prev_y;
                }
            }
        }
        codes.push(keypad[current_y as usize][current_x as usize].to_string())
    }
    println!("Solution is {}", codes.join(""));
}
