use std::fs;

fn compare_sets(first: &str, second: &str) -> bool {
    let first_split = first.split("-").collect::<Vec<&str>>();
    let second_split = second.split("-").collect::<Vec<&str>>();

    let first_start = first_split[0].parse::<i32>().unwrap();
    let first_stop = first_split[1].parse::<i32>().unwrap();

    let second_start = second_split[0].parse::<i32>().unwrap();
    let second_stop = second_split[1].parse::<i32>().unwrap();

    // Second value contained in first
    if first_start <= second_start {
        if first_stop >= second_stop {
            return true;
        }
    }

    // First value contained in second
    if first_start >= second_start {
        if first_stop <= second_stop {
            return true;
        }
    }
    return false;
}

fn set_overlap(first: &str, second: &str) -> bool {
    let first_split = first.split("-").collect::<Vec<&str>>();
    let second_split = second.split("-").collect::<Vec<&str>>();

    let first_start = first_split[0].parse::<i32>().unwrap();
    let first_stop = first_split[1].parse::<i32>().unwrap();

    let second_start = second_split[0].parse::<i32>().unwrap();
    let second_stop = second_split[1].parse::<i32>().unwrap();

    if first_start <= second_start {
        if first_stop >= second_start {
            return true;
        }
    }

    if first_start >= second_start {
        if second_stop >= first_start {
            return true;
        }
    }
    return false;
}

fn part1(lines: std::str::Lines) {
    let mut overlaps = 0;

    for line in lines {
        let split = line.split(",");
        let sets = split.collect::<Vec<&str>>();

        if compare_sets(sets[0], sets[1]) {
            overlaps += 1;
        }
    }
    println!("part1 overlaps: {}", overlaps)
}

fn part2(lines: std::str::Lines) {
    let mut overlaps = 0;

    for line in lines {
        let split = line.split(",");
        let sets = split.collect::<Vec<&str>>();

        if set_overlap(sets[0], sets[1]) {
            overlaps += 1;
        }
    }
    println!("Part2 overlaps: {}", overlaps)
}

fn main() {
    let contents = fs::read_to_string("input2.txt").expect("Should have read the file");

    let lines = contents.lines();
    part1(lines.clone());
    part2(lines.clone());

}
