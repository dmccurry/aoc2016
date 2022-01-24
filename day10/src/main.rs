use std::fs;
use std::collections::HashMap;

fn main() {
    parts();
}

fn parts() {
    // lt / ht - 0 = bot, 1 = output
    // [v1, v2, lt, l, ht, h]
    let mut bot_map: HashMap<i32, [i32; 6]> = HashMap::new();
    let mut output_map: HashMap<i32, i32> = HashMap::new();
    let input_file = fs::read_to_string("./input").expect("fail");
    let lines: Vec<&str> = input_file.split("\n").collect();

    for i in 0..lines.len() {
        let line = lines[i];
        let mut line_parts = line.split(" ");
        let s = line_parts.next().unwrap();
        if s == "value" {
            let v: i32 = line_parts.next().unwrap().parse().unwrap();
            line_parts.next(); // goes
            line_parts.next(); // to
            line_parts.next(); // bot
            let u: i32 = line_parts.next().unwrap().parse().unwrap();
            if !bot_map.contains_key(&u) {
                let tmp = [v, -1, -1, -1, -1, -1];
                bot_map.insert(u, tmp);
            } else {
                let mut tmp = *bot_map.get(&u).unwrap();
                if tmp[0] == -1 {
                    tmp[0] = v;
                } else {
                    tmp[1] = v;
                }
                bot_map.insert(u, tmp);
            }
        } else if s == "bot" {
            let u: i32 = line_parts.next().unwrap().parse().unwrap();
            line_parts.next(); // gives
            line_parts.next(); // low
            line_parts.next(); // to
            let output_or_bot1 = line_parts.next().unwrap();
            let num1: i32 = line_parts.next().unwrap().parse().unwrap();
            line_parts.next(); // and
            line_parts.next(); // high
            line_parts.next(); // to
            let output_or_bot2 = line_parts.next().unwrap();
            let num2: i32 = line_parts.next().unwrap().parse().unwrap();
            if !bot_map.contains_key(&u) {
                let mut tmp = [-1, -1, -1, -1, -1, -1];
                tmp[3] = num1;
                tmp[5] = num2;
                if output_or_bot1 == "bot" {
                    tmp[2] = 0;
                } else {
                    tmp[2] = 1;
                }
                if output_or_bot2 == "bot" {
                    tmp[4] = 0;
                } else {
                    tmp[4] = 1;
                }
                bot_map.insert(u, tmp);
            } else {
                let mut tmp = *bot_map.get(&u).unwrap();
                tmp[3] = num1;
                tmp[5] = num2;
                if output_or_bot1 == "bot" {
                    tmp[2] = 0;
                } else {
                    tmp[2] = 1;
                }
                if output_or_bot2 == "bot" {
                    tmp[4] = 0;
                } else {
                    tmp[4] = 1;
                }
                bot_map.insert(u, tmp);
            }
        }
    }
    let mut found = true;
    while found == true {
        found = false;
        let mut found_bot: i32 = -1;
        let mut updated_bots: HashMap<i32, [i32; 6]> = HashMap::new();
        for (bot, values) in &bot_map {
            if values[0] != -1 && values[1] != -1 {
                // execute the action
                found = true;
                found_bot = *bot;
                let mut low = values[0];
                let mut high = values[1];
                if low > high {
                    let tmp = high;
                    high = low;
                    low = tmp;
                }
                if low == 17 && high == 61 {
                    println!("Solution is {}", bot);
                }
                if values[2] == 0 {
                    let mut other_bot = *bot_map.get(&values[3]).unwrap();
                    if other_bot[0] == -1 {
                        other_bot[0] = low;
                    } else {
                        other_bot[1] = low;
                    }
                    updated_bots.insert(values[3], other_bot);
                } else {
                    output_map.insert(values[3], low);
                }
                if values[4] == 0 {
                    let mut other_bot = *bot_map.get(&values[5]).unwrap();
                    if other_bot[0] == -1 {
                        other_bot[0] = high;
                    } else {
                        other_bot[1] = high;
                    }
                    updated_bots.insert(values[5], other_bot);
                } else {
                    output_map.insert(values[5], high);
                }
                break;
            }
        }

        bot_map.remove(&found_bot);
        for (bot, values) in updated_bots {
            bot_map.insert(bot, values);
        }
    }
    
    let zero = output_map.get(&0).unwrap();
    let one = output_map.get(&1).unwrap();
    let two = output_map.get(&2).unwrap();

    println!("part two is {}", zero * one * two);
}
