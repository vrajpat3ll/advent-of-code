use std::{fs, thread};

fn part1(bank: &str) -> u64 {
    let mut joltage: u64 = 0;
    for i in 0..(bank.chars().count() - 1) {
        if bank.as_bytes()[i] == '\r' as u8 {
            continue;
        }
        for j in (i + 1)..bank.chars().count() {
            let a: u64 = (bank.as_bytes()[i] - '0' as u8) as u64;
            let b: u64 = (bank.as_bytes()[j] - '0' as u8) as u64;
            let j = 10 * a + b;
            if j > joltage {
                joltage = j;
            }
        }
    }
    joltage
}

fn max_joltage_by_selecting_k_batteries(
    bank: &str,
    k: u8,
    curr_index: usize,
    picked: u8,
    current: u64,
) -> u64 {
    if picked >= k || curr_index >= bank.chars().count() {
        return current;
    }

    let curr_digit = bank.as_bytes()[curr_index];
    let number = 10 * current + (curr_digit - '0' as u8) as u64;

    let taken = max_joltage_by_selecting_k_batteries(bank, k, curr_index + 1, picked + 1, number);

    let not_taken = max_joltage_by_selecting_k_batteries(bank, k, curr_index + 1, picked, current);
    return if taken > not_taken { taken } else { not_taken };
}

fn part2(bank: &str) -> u64 {
    max_joltage_by_selecting_k_batteries(bank, 12, 0, 0, 0)
}

fn solve(content: &String, part: u8) -> u64 {
    let solver: fn(&str) -> u64;
    match part {
        1 => solver = part1,
        2 => solver = part2,
        _ => panic!("incorrect part number!"),
    }
    let mut answer: u64 = 0;
    let banks = content.split("\r\n");

    // way too slow for part 2
    for bank in banks {
        let joltage = solver(bank);
        answer += joltage as u64;
        dbg!(joltage);
    }
    answer
}

fn solve_parallel(content: &String, part: u8) -> u64 {
    let solver: fn(&str) -> u64;
    match part {
        1 => solver = part1,
        2 => solver = part2,
        _ => panic!("incorrect part number!"),
    }
    let mut answer: u64 = 0;
    let banks = content.split("\r\n");
    let mut handles = Vec::new();

    for bank in banks {
        let bnk = String::from(bank);
        let handle = thread::spawn(move || solver(&bnk));
        handles.push(handle);
    }
    for handle in handles {
        let a = handle.join().unwrap();
        answer += a;
    }
    answer
}

fn main() {
    let content = fs::read_to_string("../inputs/3.txt").expect("incorrect file name");
    dbg!(solve(&content, 1));
    dbg!(solve(&content, 2));
}
