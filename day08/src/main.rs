use std::fs;

fn main() {
    part1();
}

fn part1() {
    let input_file = fs::read_to_string("./input").expect("error");
    const width: usize = 50;
    const height: usize = 6;
    let mut grid = [['.'; width]; height];
    let lines = input_file.split("\n");
    for line in lines {
        let mut instructions = line.split(" ");
        let instruction = instructions.next().unwrap();
        let instruction2 = instructions.next().unwrap();
        if instruction == "rect" {
            let mut width_and_height = instruction2.split("x");
            let rect_width: usize = width_and_height.next().unwrap().parse().unwrap();
            let rect_height: usize = width_and_height.next().unwrap().parse().unwrap();
            for i in 0..rect_height {
                for j in 0..rect_width {
                    grid[i][j] = '#';
                }
            }
        } else if instruction2 == "row" {
            let start_s = instructions.next().unwrap();
            let by = instructions.next().unwrap();
            let amount: usize = instructions.next().unwrap().parse().unwrap();
            let start: usize = start_s.replace("y=", "").parse().unwrap();
            let mut new_row = ['.'; width];
            for i in 0..width {
                if grid[start][i] == '#' {
                    let mut target_location = i + amount;
                    if target_location >= width {
                        target_location -= width;
                    }
                    new_row[target_location] = '#';
                }
            }
            for i in 0..width {
                grid[start][i] = new_row[i]
            }
        } else if instruction2 == "column" {
            let start_s = instructions.next().unwrap();
            let by = instructions.next().unwrap();
            let amount: usize = instructions.next().unwrap().parse().unwrap();
            let start: usize = start_s.replace("x=", "").parse().unwrap();
            let mut new_col = ['.'; height];
            for i in 0..height {
                if grid[i][start] == '#' {
                    let mut target_location = i + amount;
                    if target_location >= height {
                        target_location -= height;
                    }
                    new_col[target_location] = '#';
                }
            }
            for i in 0..height {
                grid[i][start] = new_col[i]
            }
        }
    }
    let mut n_lit = 0;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '#' {
                n_lit += 1;
            }
        }
    }
    println!("{}", n_lit);

    // part 2
    let mut s = String::from("");
    for i in 0..height {
        for j in 0..width {
            s.push(grid[i][j]);
        }
        s.push('\n');
    }
    println!("{}", s);
}
