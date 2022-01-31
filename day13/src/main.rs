use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn is_open(x: usize, y: usize, num: usize) -> usize {
    let sum = x*x + 3*x + 2*x*y + y + y*y + num;
    let bin = format!("{:b}", sum);
    let c = bin.matches("1").count();
    return c % 2;
}

fn part1() {
    let fave = 1352;
    let target_x = 31;
    let target_y = 39;
    let mut queue: Vec<String> = Vec::new();
    let mut dist: HashMap<String, usize> = HashMap::new();

    queue.push(String::from("1,1"));
    dist.insert(String::from("1,1"), 0);

    while queue.len() > 0 {
        let next: String = queue.remove(0);
        let xy: Vec<&str> = next.split(",").collect();
        let x: usize = xy[0].parse().unwrap();
        let y: usize = xy[1].parse().unwrap();
        let d: usize = *dist.get(&next).unwrap();

        if x == target_x && y == target_y {
            let end_key: String = String::from(target_x.to_string() + "," + &target_y.to_string());
            println!("{:?}", dist.get(&end_key));
            break;
        }

        if x > 0 {
            let key: String = String::from((x-1).to_string() + "," + &y.to_string());
            let key1: String = String::from((x-1).to_string() + "," + &y.to_string());
            if is_open(x - 1, y, fave) == 0 && !dist.contains_key(&key) {
                dist.insert(key, d+1);
                queue.push(key1);
            }
        }
        if y > 0 {
            let key: String = String::from(x.to_string() + "," + &(y-1).to_string());
            let key1: String = String::from(x.to_string() + "," + &(y-1).to_string());
            if is_open(x, y-1, fave) == 0 && !dist.contains_key(&key) {
                dist.insert(key, d+1);
                queue.push(key1);
            }
        }

        let mut key2: String = String::from((x+1).to_string() + "," + &y.to_string());
        let mut key3: String = String::from((x+1).to_string() + "," + &y.to_string());
        if is_open(x+1, y, fave) == 0 && !dist.contains_key(&key2) {
            dist.insert(key2, d+1);
            queue.push(key3);
        }

        key2 = String::from(x.to_string() + "," + &(y+1).to_string());
        key3 = String::from(x.to_string() + "," + &(y+1).to_string());
        if is_open(x, y+1, fave) == 0 && !dist.contains_key(&key2) {
            dist.insert(key2, d+1);
            queue.push(key3);
        }
    }
}

fn part2() {
    let fave = 1352;
    let target_x = 31;
    let target_y = 39;
    let mut queue: Vec<String> = Vec::new();
    let mut dist: HashMap<String, usize> = HashMap::new();

    queue.push(String::from("1,1"));
    dist.insert(String::from("1,1"), 0);

    while queue.len() > 0 {
        let next: String = queue.remove(0);
        let xy: Vec<&str> = next.split(",").collect();
        let x: usize = xy[0].parse().unwrap();
        let y: usize = xy[1].parse().unwrap();
        let d: usize = *dist.get(&next).unwrap();

        if d >= 50 {
            println!("{}", dist.len());
            break;
        }

        if x > 0 {
            let key: String = String::from((x-1).to_string() + "," + &y.to_string());
            let key1: String = String::from((x-1).to_string() + "," + &y.to_string());
            if is_open(x - 1, y, fave) == 0 && !dist.contains_key(&key) {
                dist.insert(key, d+1);
                queue.push(key1);
            }
        }
        if y > 0 {
            let key: String = String::from(x.to_string() + "," + &(y-1).to_string());
            let key1: String = String::from(x.to_string() + "," + &(y-1).to_string());
            if is_open(x, y-1, fave) == 0 && !dist.contains_key(&key) {
                dist.insert(key, d+1);
                queue.push(key1);
            }
        }

        let mut key2: String = String::from((x+1).to_string() + "," + &y.to_string());
        let mut key3: String = String::from((x+1).to_string() + "," + &y.to_string());
        if is_open(x+1, y, fave) == 0 && !dist.contains_key(&key2) {
            dist.insert(key2, d+1);
            queue.push(key3);
        }

        key2 = String::from(x.to_string() + "," + &(y+1).to_string());
        key3 = String::from(x.to_string() + "," + &(y+1).to_string());
        if is_open(x, y+1, fave) == 0 && !dist.contains_key(&key2) {
            dist.insert(key2, d+1);
            queue.push(key3);
        }
    }
}