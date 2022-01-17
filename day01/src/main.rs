use std::fs;
use std::collections::HashSet;
fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("./input")
        .expect("fail");

    let dirs = contents.split(", ");
    let mut current_dir = "N";
    let mut current_x = 0;
    let mut current_y = 0;

    for dir in dirs {
        let dest = dir.chars().nth(0).unwrap();
        let mut chars = dir.chars();
        chars.next();
        let n: i32 = chars.as_str().parse().unwrap();
        if dest == 'R' {
            if current_dir == "N" {
                current_dir = "E";
                current_x = current_x + n;
            } else if current_dir == "E" {
                current_dir = "S";
                current_y = current_y - n;
            } else if current_dir == "S" {
                current_dir = "W";
                current_x = current_x - n;
            } else if current_dir == "W" {
                current_dir = "N";
                current_y = current_y + n;
            }
        } else {
            if current_dir == "N" {
                current_dir = "W";
                current_x = current_x - n;
            } else if current_dir == "E" {
                current_dir = "N";
                current_y = current_y + n;
            } else if current_dir == "S" {
                current_dir = "E";
                current_x = current_x + n;
            } else if current_dir == "W" {
                current_dir = "S";
                current_y = current_y - n;
            }
        }
    }

    let solution = current_x.abs() + current_y.abs();

    println!("Solution is: {}", solution)
}

fn part2() {
    let contents = fs::read_to_string("./input")
        .expect("fail");

    let dirs = contents.split(", ");
    let mut current_dir = "N";
    let mut current_x = 0;
    let mut current_y = 0;
    let mut visited = HashSet::<String>::new();
    let mut done = false;
    for dir in dirs {
        if done == true {
            break;
        }
        let dest = dir.chars().nth(0).unwrap();
        let mut chars = dir.chars();
        chars.next();
        let n: i32 = chars.as_str().parse().unwrap();
        let mut next_x = current_x;
        let mut next_y = current_y;
        if dest == 'R' {
            if current_dir == "N" {
                current_dir = "E";
                next_x = current_x + n;
            } else if current_dir == "E" {
                current_dir = "S";
                next_y = current_y - n;
            } else if current_dir == "S" {
                current_dir = "W";
                next_x = current_x - n;
            } else if current_dir == "W" {
                current_dir = "N";
                next_y = current_y + n;
            }
        } else {
            if current_dir == "N" {
                current_dir = "W";
                next_x = current_x - n;
            } else if current_dir == "E" {
                current_dir = "N";
                next_y = current_y + n;
            } else if current_dir == "S" {
                current_dir = "E";
                next_x = current_x + n;
            } else if current_dir == "W" {
                current_dir = "S";
                next_y = current_y - n;
            }
        }

        loop {

            if next_x > current_x {
                current_x = current_x + 1;
            } else if next_x < current_x {
                current_x -= 1;
            }

            if next_y > current_y {
                current_y += 1;
            } else if next_y < current_y {
                current_y -= 1;
            }

            let key = "".to_owned() + &current_x.to_string() + "," + &current_y.to_string() + ")";

            if visited.contains(&key) {
                let solution = current_x.abs() + current_y.abs();
                println!("Solution is: {}", solution);
                done = true;
                break;
            }
            visited.insert(key);

            if next_x == current_x && next_y == current_y {
                break;
            }
        }
    }
}