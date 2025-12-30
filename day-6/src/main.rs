use core::panic;
use std::fs;

#[derive(Debug)]
struct Answer(u128);

fn add(a: u128, b: u128) -> u128 {
    a + b
}

fn mult(a: u128, b: u128) -> u128 {
    a * b
}

fn solve_part1(s: &str) -> Answer {
    let split: Vec<&str> = s.split("\r\n").collect();
    let ops: Vec<&str> = split[split.len() - 1]
        .split(" ")
        .filter(|x| *x != "")
        .collect();
    let mut answer_store: Vec<Vec<u128>> = vec![];

    let mut answer = 0;
    for i in 0..(split.len() - 1) {
        let line = split[i];
        let values: Vec<&str> = line.split(" ").collect();
        // dbg!(&values);
        let values: Vec<u128> = values
            .iter()
            .filter(|x| **x != "")
            .map(|x| (**x).parse::<u128>().expect("unknown value"))
            .collect();
        answer_store.push(values);
    }

    for (i, op) in ops.iter().enumerate() {
        let func: fn(u128, u128) -> u128;
        let mut temp = 0;
        match *op {
            "+" => func = add,
            "*" => {
                func = mult;
                temp = 1
            }
            _ => panic!("illegal operation found!"),
        }
        for j in 0..answer_store.len() {
            // dbg!(&j, &temp, &answer_store[j][i]);
            temp = func(temp, answer_store[j][i]);
        }
        answer += temp;
    }
    Answer(answer)
}

fn solve_part2(s: &str) -> Answer {
    let split: Vec<&str> = s.split("\r\n").collect();
    let ops: Vec<&str> = split[split.len() - 1]
        .split(" ")
        .filter(|x| *x != "")
        .collect();
    let content: Vec<&&str> = split[..split.len() - 1].iter().collect();

    let mut cnt = 0;
    let mut answer = 0;
    let mut dummy: Vec<u128> = vec![];
    for j in (0..content[0].len()).rev() {
        let mut tmp = 0;
        for i in 0..content.len() {
            let c = content[i].as_bytes()[j];
            match c {
                b'0'..=b'9' => {
                    tmp = 10 * tmp + c as u128 - ('0' as u128);
                }
                _ => {}
            }
        }
        if tmp == 0 {
            let mut temp = 0;
            let func: fn(u128, u128) -> u128;
            let op = ops[ops.len() - 1 - cnt];
            match op {
                "+" => func = add,
                "*" => {
                    func = mult;
                    temp = 1
                }
                _ => panic!("illegal operation found!"),
            }
            for v in &dummy {
                temp = func(temp, *v);
            }
            answer += temp;
            cnt += 1;
            // dbg!(&dummy, &temp, &op);
            dummy.clear();
        } else {
            dummy.push(tmp);
        }
    }
    {
        // first operation remains
        let mut temp = 0;
        let func: fn(u128, u128) -> u128;
        let op = ops[0];
        match op {
            "+" => func = add,
            "*" => {
                func = mult;
                temp = 1
            }
            _ => panic!("illegal operation found!"),
        }
        for v in &dummy {
            temp = func(temp, *v);
        }
        answer += temp;
    }
    Answer(answer)
}

fn solve(s: &str, part: u8) -> Answer {
    let solver: fn(&str) -> Answer;
    match part {
        1 => solver = solve_part1,
        2 => solver = solve_part2,
        _ => panic!("incorrect part number!"),
    }
    solver(s)
}

fn main() {
    let content = fs::read_to_string("../inputs/6.txt").expect("incorrect file name!");
    // let content = fs::read_to_string("input.txt").expect("incorrect file name!");
    dbg!(solve(&content, 1).0);
    dbg!(solve(&content, 2).0);
}
