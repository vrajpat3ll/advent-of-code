use std::fs;

fn part1(content: &str) -> i32 {
    let mut answer: i32 = 0;
    let mut pos: i32 = 50;
    let mut dir: i32;
    let total_len = content.chars().count();
    for mut i in 0..total_len {
        let c = content.as_bytes()[i] as char;
        match c {
            '\n' => continue,
            'L' => dir = -1,
            'R' => dir = 1,
            _ => continue,
        }
        let mut number = 0;
        i += 1;
        while i < total_len {
            let c = content.as_bytes()[i] as char;
            match c {
                '0'..='9' => number = 10 * number + (c as i32 - '0' as i32),
                _ => break,
            }
            i += 1;
        }
        pos += dir * number;
        pos = (pos + 100) % 100;
        if pos == 0 {
            answer += 1;
        }
        // let dirchr = if dir == -1 { 'L' } else { 'R' };
        // println!("{dirchr}{number}");
    }
    return answer;
}

fn part2(content: &str) -> i32 {
    let mut answer: i32 = 0;
    let mut pos: i32 = 50;
    let mut dir: i32;
    let total_len = content.chars().count();
    for mut i in 0..total_len {
        let c = content.as_bytes()[i] as char;
        match c {
            '\n' => continue,
            'L' => dir = -1,
            'R' => dir = 1,
            _ => continue,
        }
        let mut number = 0;
        i += 1;
        while i < total_len {
            let c = content.as_bytes()[i] as char;
            match c {
                '0'..='9' => number = 10 * number + (c as i32 - '0' as i32),
                _ => break,
            }
            i += 1;
        }

        // direction to rotate in, rotations/clicks, current position => this is all we have
        for _ in 1..=number {
            pos += dir;
            pos = (pos + 100) % 100;
            if pos == 0 {
                answer += 1;
            }
        }
    }
    return answer;
}

fn main() {
    let content = fs::read_to_string("../inputs/1.txt").expect("Incorrect file path");

    dbg!(part1(&content));
    dbg!(part2(&content));
}
