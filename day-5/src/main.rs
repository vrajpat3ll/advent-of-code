use std::fs;

#[derive(Debug)]
struct Answer(u128);

fn solve_part1(s: &str) -> Answer {
    let split: Vec<&str> = s.split("\r\n\r\n").collect();
    let ranges: Vec<&str> = split[0].split("\r\n").collect();
    let ids_str: Vec<&str> = split[1].split("\r\n").collect();
    let mut answer = 0;

    for id in ids_str {
        for range in &ranges {
            let start_end: Vec<&str> = range.split("-").collect();
            let start: u128 = start_end[0].parse().unwrap();
            let end: u128 = start_end[1].parse().unwrap();
            let id = id.parse().unwrap();

            if start <= id && id <= end {
                answer += 1;
                break;
            }
        }
    }
    Answer(answer)
}

fn contains(interval: &mut (u128, u128), sub_interval: &mut (u128, u128)) -> bool {
    interval.0 <= sub_interval.0 && interval.1 >= sub_interval.1
}

fn merge_intervals(intervals: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    if intervals.len() <= 1 {
        return intervals;
    }

    let mut intervals = intervals.clone();
    intervals.sort();

    let n = intervals.len();
    let mut answer: Vec<(u128, u128)> = vec![];
    answer.push(intervals[0]);

    for i in 1..n {
        let index = answer.len() - 1;
        let back = &mut answer[index];

        if back.1 >= intervals[i].0 && back.0 <= intervals[i].0 && back.1 < intervals[i].1 {
            back.1 = intervals[i].1;
        } else if contains(back, &mut intervals[i]) {
            continue;
        } else if contains(&mut intervals[i], back) {
            back.0 = intervals[i].0;
            back.1 = intervals[i].1;
        } else {
            answer.push(intervals[i]);
        }
    }
    answer
}

fn solve_part2(s: &str) -> Answer {
    // need to merge intervals first, then calculate total
    let split: Vec<&str> = s.split("\r\n\r\n").collect();
    let ranges: Vec<&str> = split[0].split("\r\n").collect();
    let mut intervals: Vec<(u128, u128)> = vec![];

    for range in &ranges {
        let start_end: Vec<&str> = range.split("-").collect();
        let start: u128 = start_end[0].parse().unwrap();
        let end: u128 = start_end[1].parse().unwrap();
        intervals.push((start, end));
    }

    let intervals = merge_intervals(intervals);
    let mut answer: u128 = 0;
    for (start, end) in intervals {
        answer += end - start + 1;
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
    let content = fs::read_to_string("../inputs/5.txt").expect("incorrect file name!");
    // let content = fs::read_to_string("test/input.txt").expect("incorrect file name!");
    dbg!(solve(&content, 1).0);
    dbg!(solve(&content, 2).0);
}
