use core::panic;
use std::fs;

#[derive(Debug)]
struct Answer(u128);

fn solve_part1(s: &str) -> Answer {
    let mut lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let mut answer = 0;
    for i in 0..(lines.len() - 1) {
        // println!("-------------------------------------------------------------------------------");
        // dbg!(&i);
        // dbg!(&lines);
        if i == 0 {
            let _index = lines[0]
                .find('S')
                .expect("There should have been an S in the first line!");
            unsafe {
                lines[1].as_bytes_mut()[_index] = b'|';
            }
            // dbg!(&lines);
            continue;
        }
        let (line, next_line) = {
            let (left, right) = lines.split_at_mut(i + 1);
            (&left[i], &mut right[0])
        };
        let matches: Vec<usize> = line.match_indices("|").map(|(i, _)| i).collect();
        for _match in matches {
            if next_line.as_bytes()[_match] == b'^' {
                let bytes = unsafe { (*next_line).as_bytes_mut() };
                if 0 < _match {
                    // println!("[^][0 < {_match}] here");
                    bytes[_match - 1] = b'|';
                }
                if _match < line.len() {
                    // println!("[^][{_match} < {}] here", line.len());
                    bytes[_match + 1] = b'|';
                }
                answer += 1;
            } else if next_line.as_bytes()[_match] == b'.' {
                let bytes = unsafe { (*next_line).as_bytes_mut() };
                // println!("[.][{_match}] here");
                bytes[_match] = b'|';
            }
        }
    }
    dbg!(&lines);
    Answer(answer)
}

fn solve_part2(s: &str) -> Answer {
    let _: Vec<Vec<u8>> = s.lines().map(|l| l.as_bytes().to_vec()).collect();

    let answer = 0;
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
    // let content = fs::read_to_string("../inputs/7.txt").expect("incorrect file name!");
    let content = fs::read_to_string("input.txt").expect("incorrect file name!");
    dbg!(solve(&content, 1).0);
    dbg!(solve(&content, 2).0);
}
