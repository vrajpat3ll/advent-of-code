use std::fs;

fn parse_number(s: &str, index: &mut usize) -> (u64, usize) {
    let mut number: u64 = 0;
    while *index < s.chars().count() {
        let c = s.as_bytes()[*index] as char;
        match c {
            '0'..='9' => number = 10 * number + c as u64 - '0' as u64,
            _ => break,
        }
        *index += 1;
    }
    (number, *index)
}

fn valid_part1(_id: u64) -> bool {
    let id = ToString::to_string(&_id);
    let len = id.chars().count();
    if len % 2 == 0 {
        let p1 = String::from(&id[0..(len / 2)]);
        let p2 = String::from(&id[(len / 2)..]);
        // dbg!(&p1, &p2);
        return p1 != p2;
    }
    return true;
}

fn valid_part2(_id: u64) -> bool {
    let id = ToString::to_string(&_id);
    let len = id.chars().count();
    let mut flag = false;

    for i in 1..=len / 2 {
        if len % (len / i) == 0 {
            let p1 = String::from(&id[0..i]);
            let mut part_times_k = String::from("");
            for _ in 0..(len / i) {
                part_times_k.push_str(&p1);
            }

            flag |= part_times_k == id;
        }
        if flag {
            return false;
        }
    }
    return true;
}
fn solve(content: &str, part: u8) -> u64 {
    let len = content.chars().count();
    // println!("Hello, world!");
    let mut answer = 0;
    let mut i: usize = 0;
    while i < len {
        let (low, index) = parse_number(&content, &mut i); // 134535
        i = index + 1; // -
        let (high, index) = parse_number(&content, &mut i); // 23143
        i = index + 1; // ,
        let mut valid: fn(u64) -> bool;
        for id in low..=high {
            match part {
                1 => valid = valid_part1,
                2 => valid = valid_part2,
                _ => panic!("incorrect part number"),
            }
            if !valid(id) {
                answer += id;
            }
        }
    }
    answer
}

fn main() {
    let content = fs::read_to_string("../inputs/2.txt").expect("incorrect file name");
    dbg!(solve(&content, 1));
    dbg!(solve(&content, 2));
}
