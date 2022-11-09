use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input_file = fs::read_to_string("./input").expect("fail");
    let mut positions: Vec<usize> = Vec::new();
    let mut at: Vec<usize> = Vec::new();

    for line in input_file.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        positions.push(parts[3].parse().unwrap());
        at.push(parts[11].replace(".", "").parse().unwrap());
    }

    let mut through = 0;
    let mut time = 0;

    while through == 0 {
        through = 1;

        for i in 0..positions.len() {
            let total_time = 1 + i + time;
            let total_ticks = total_time + at.get(i).unwrap();
            if total_ticks % positions.get(i).unwrap() != 0 {
                through = 0;
            }
        }

        time += 1;
    }
    println!("{}", time - 1);

}

fn part2() {
    let input_file = fs::read_to_string("./input").expect("fail");
    let mut positions: Vec<usize> = Vec::new();
    let mut at: Vec<usize> = Vec::new();

    for line in input_file.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        positions.push(parts[3].parse().unwrap());
        at.push(parts[11].replace(".", "").parse().unwrap());
    }

    positions.push(11); 
    at.push(0);

    let mut through = 0;
    let mut time = 0;

    while through == 0 {
        through = 1;

        for i in 0..positions.len() {
            let total_time = 1 + i + time;
            let total_ticks = total_time + at.get(i).unwrap();
            if total_ticks % positions.get(i).unwrap() != 0 {
                through = 0;
            }
        }

        time += 1;
    }
    println!("{}", time - 1);

}
