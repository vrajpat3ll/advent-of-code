use std::fs;
const THRESHOLD: u64 = 4;

fn vecs_to_string(data: Vec<Vec<u8>>) -> Result<String, std::string::FromUtf8Error> {
    let mut result = String::new();

    for inner_vec in data {
        // Attempt to convert the inner Vec<u8> to a String
        match String::from_utf8(inner_vec) {
            Ok(s) => {
                result.push_str(&s);
                result.push_str("\r\n"); // Add the newline character
            }
            Err(e) => {
                // Return an error if any inner vector contains invalid UTF-8
                return Err(e);
            }
        }
    }

    Ok(result)
}

fn count_surroundings(curr: &str, prev: &str, next: &str, index: usize) -> u64 {
    if index >= curr.chars().count() {
        return THRESHOLD + 1;
    }
    if curr.as_bytes()[index] != b'@' {
        return THRESHOLD + 1;
    }
    let mut answer = 0;
    if index > 0 {
        if prev.as_bytes()[index - 1] == b'@' {
            answer += 1;
        }
        if curr.as_bytes()[index - 1] == b'@' {
            answer += 1;
        }
        if next.as_bytes()[index - 1] == b'@' {
            answer += 1;
        }
    }
    if index < curr.chars().count() - 1 {
        if prev.as_bytes()[index + 1] == b'@' {
            answer += 1;
        }
        if curr.as_bytes()[index + 1] == b'@' {
            answer += 1;
        }

        if next.as_bytes()[index + 1] == b'@' {
            answer += 1;
        }
    }
    if prev.as_bytes()[index] == b'@' {
        answer += 1;
    }
    if next.as_bytes()[index] == b'@' {
        answer += 1;
    }

    answer
}

fn solve_part1(s: &str) -> (u64, String) {
    let splits: Vec<&str> = s.split("\r\n").collect();
    let mut grid: Vec<Vec<u8>> = s.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut answer = 0;

    for i in 0..splits.len() {
        let curr = splits[i];
        let prev: String;
        if i == 0 {
            prev = std::iter::repeat(".").take(curr.len()).collect();
        } else {
            prev = String::from(splits[i - 1]);
        }
        let next;
        if i == (splits.len() - 1) {
            next = std::iter::repeat(".").take(curr.len()).collect();
        } else {
            next = String::from(splits[i + 1]);
        }
        for j in 0..curr.chars().count() {
            if count_surroundings(curr, &prev, &next, j) < THRESHOLD {
                answer += 1;
                grid[i][j] = b'x';
            }
            // println!("-----------------------");
            // println!("j = {j}");
            // println!("prev = |{prev}|");
            // println!("curr = |{curr}|");
            // println!("next = |{next}|");
            // println!("-----------------------");
        }
    }
    let x = vecs_to_string(grid).expect("hi");
    (answer, x)
}

fn solve_part2(s: &str) -> (u64, String) {
    let mut wall: String = String::from(s);
    let mut answer = 0;
    loop {
        // println!("wall =\n{wall}\n---\n{answer}");
        let iter = solve_part1(&wall);
        wall = iter.1;
        wall = wall.strip_suffix("\r\n").expect("could not find \\r\\n in th end").to_string();
        answer += iter.0;
        if iter.0 == 0 {
            break;
        }
    }

    (answer, wall)
}

fn solve(s: &str, part: u8) -> u64 {
    let solver: fn(&str) -> (u64, String);
    match part {
        1 => solver = solve_part1,
        2 => solver = solve_part2,
        _ => panic!("incorrect part number!"),
    }
    solver(s).0
}

fn main() {
    let content = fs::read_to_string("../inputs/4.txt").expect("incorrect file name!");
    // let content = fs::read_to_string("test/input.txt").expect("incorrect file name!");
    dbg!(solve(&content, 1));
    dbg!(solve(&content, 2));
}
