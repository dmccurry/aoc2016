fn main() {
    part1();
    part2();
}

fn part1() {
    let start: String = "hhhxzeay".to_string();
    let path: String = "".to_string();
    let x = 0;
    let y = 0;
    let mut valid_paths: Vec<String> = Vec::new();
    next(start, path, x, y, &mut valid_paths);
    print_shortest_path(&mut valid_paths);
}

fn part2() {
    let start: String = "hhhxzeay".to_string();
    let path: String = "".to_string();
    let x = 0;
    let y = 0;
    let mut valid_paths: Vec<String> = Vec::new();
    next(start, path, x, y, &mut valid_paths);
    print_longest_path(&mut valid_paths);
}

fn next(start: String, path: String, x: usize, y: usize, valid_paths: &mut Vec<String>) {

    if x == 3 && y == 3 {
        valid_paths.push(path);
        return;
    }
    
    let digest = md5::compute(start.to_owned() + &path.to_owned());
    let s = format!("{:x}", digest);
    let chars: Vec<_> = s.chars().collect();
    let up = y != 0 && open_char(*chars.get(0).unwrap());
    let down = y != 3 && open_char(*chars.get(1).unwrap());
    let left = x != 0 && open_char(*chars.get(2).unwrap());
    let right = x != 3 && open_char(*chars.get(3).unwrap());
    if !up && !down && !left && !right {
        return;
    }

    if up {
        next(start.clone(), path.clone() + "U", x, y - 1, valid_paths);
    }
    if down {
        next(start.clone(), path.clone() + "D", x, y + 1, valid_paths);
    }
    if left {
        next(start.clone(), path.clone() + "L", x - 1, y, valid_paths);
    }
    if right {
        next(start.clone(), path.clone() + "R", x + 1, y, valid_paths);
    }
}

fn open_char(c: char) -> bool {
    return c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f';
}

fn print_shortest_path(paths: &mut Vec<String>) {
    let mut shortest = paths[0].clone();

    for n in 0..paths.len() {
        if paths[n].len() < shortest.len() {
            shortest = paths[n].clone();
        }
    }

    println!("{}", shortest);
}

fn print_longest_path(paths: &mut Vec<String>) {
    let mut longest = paths[0].clone();

    for n in 0..paths.len() {
        if paths[n].len() > longest.len() {
            longest = paths[n].clone();
        }
    }

    println!("{}", longest.len());
}
