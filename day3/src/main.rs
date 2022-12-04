use std::fs;


fn part1(lines: std::str::Lines) {
    let mut tot = 0;
    for line in lines {
        let first = &line[..line.len()/2];
        let second = &line[line.len()/2..];
        //println!("First: {}", first);
        //println!("Second: {}", second);
        let mut shared = vec![];
        for letter in first.chars() {
            if second.contains(letter) {
                shared.push(letter);
            }
        }
        shared.dedup();
        for letter in shared {
            let tmp = convert_to_prio(letter);
            tot += tmp;
            println!("Line: {}, Shared: {}, Value: {}", line, letter, tmp);
        }
    }
    println!("Total: {}", tot);
}

fn convert_to_prio(letter: char) -> u32 {

    let mut temp = letter as u32;
    if letter.is_ascii_lowercase() {
        temp = temp - 96;
    } else if letter.is_ascii_uppercase() {
        temp = temp - 38;
    }
    return temp;
}

fn part2(lines: std::str::Lines) {
    let mut tot = 0;
    let mut line_nr = 1;
    let mut tmp_lines = vec![];
    for line in lines {
        tmp_lines.push(line);
        if line_nr % 3 == 0 {
            let mut shared = vec![];
            for letter in tmp_lines[0].chars() {
                let tmp1 = tmp_lines[1].to_string();
                if tmp1.contains(letter) {
                    let tmp2 = tmp_lines[2].to_string();
                        if tmp2.contains(letter) {
                            shared.push(letter);
                        }
                }
            }
            shared.dedup();
            for letter in shared {
                let tmp = convert_to_prio(letter);
                tot += tmp;
                println!("Line: {}, Shared: {}, Value: {}", line, letter, tmp);
            }
            tmp_lines.clear();
        }
        line_nr+=1;
    }
    println!("Total2: {}", tot);
}


fn main() {
    let contents = fs::read_to_string("input2.txt").expect("Should have read the file");

    let lines = contents.lines();
    part1(lines.clone());
    part2(lines.clone());
}
