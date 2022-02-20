fn main() {
    part1();
    part2();
}

fn part2() {
    let mut hashes: Vec<String> = Vec::new();
    let salt = "ahsbgdzn";
    let mut hash_size: usize = 0;
    while hash_size < 30000 {
        let mut start = salt.to_owned() + &hash_size.to_string();
        for h in 1..2018 {
            let digest = md5::compute(start);
            let s = format!("{:x}", digest);
            start = s;
        }
        hashes.push(start);
        hash_size += 1;
    }


    let mut index: usize = 0;
    let mut keys = 1;
    let mut has_triplet;

    while keys < 65 {
        let chars: Vec<_> = hashes.get(index).unwrap().chars().collect();
        has_triplet = 0;
        for n in 0..chars.len() - 2 {
            let a = chars.get(n).unwrap();
            let b = chars.get(n+1).unwrap();
            let c = chars.get(n+2).unwrap();
            if a == b && a == c && has_triplet == 0 {
                has_triplet = 1;
                let v = vec![a, a, a, a, a];
                let r: String = v.into_iter().collect();
                // check next 1000
                for m in 1..1000 {
                    let new_start = hashes.get(index+m).unwrap();
                    if new_start.contains(&r) {
                        keys += 1;
                        break;
                    }
                }
            }
        }
        index += 1;
    }
    println!("{}", index-1);
}

fn part1() {
    let salt = "ahsbgdzn";
    let mut index: usize = 0;
    let mut keys = 1;
    let mut has_triplet = 0;

    while keys < 64 {
        let digest = md5::compute(salt.to_owned() + &index.to_string());
        let s = format!("{:x}", digest);
        let chars: Vec<_> = s.chars().collect();
        has_triplet = 0;
        for n in 0..chars.len() - 3 {
            let a = chars.get(n).unwrap();
            let b = chars.get(n+1).unwrap();
            let c = chars.get(n+2).unwrap();
            if a == b && a == c && has_triplet == 0 {
                has_triplet = 1;
                let v = vec![a, a, a, a, a];
                let r: String = v.into_iter().collect();
                // check next 1000
                for m in 1..1000 {
                    let new_digest = md5::compute(salt.to_owned() + &(m+index).to_string());
                    let t = format!("{:x}", new_digest);
                    if t.contains(&r) {
                        keys += 1;
                        break;
                    }
                }
            }
        }
        index += 1;
    }

    println!("{}", index-1)
}
