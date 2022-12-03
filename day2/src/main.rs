use std::fs;

enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Results {
    Win = 6,
    Draw = 3,
    Lost = 0,
}

fn calculate_score_part1(op_move: &str, my_move: &str) -> i32 {
    let mut score = 0;
    match my_move {
        "X" => {
            score += Moves::Rock as i32;
            match op_move {
                "A" => {
                    score += Results::Draw as i32;
                }
                "B" => {
                    score += Results::Lost as i32;
                }
                "C" => {
                    score += Results::Win as i32;
                }
                _ => {}
            }
        }
        "Y" => {
            score += Moves::Paper as i32;
            match op_move {
                "A" => {
                    score += Results::Win as i32;
                }
                "B" => {
                    score += Results::Draw as i32;
                }
                "C" => {
                    score += Results::Lost as i32;
                }
                _ => {}
            }
        }
        "Z" => {
            score += Moves::Scissors as i32;
            match op_move {
                "A" => {
                    score += Results::Lost as i32;
                }
                "B" => {
                    score += Results::Win as i32;
                }
                "C" => {
                    score += Results::Draw as i32;
                }
                _ => {}
            }
        }
        _ => {}
    }

    return score;
}
fn calculate_score_part2(op_move: &str, my_move: &str) -> i32 {
    let mut score = 0;
    match my_move {
        "X" => {
            score += Results::Lost as i32;
            match op_move {
                "A" => {
                    score += Moves::Scissors as i32;
                }
                "B" => {
                    score += Moves::Rock as i32;
                }
                "C" => {
                    score += Moves::Paper as i32;
                }
                _ => {}
            }
        }
        "Y" => {
            score += Results::Draw as i32;
            match op_move {
                "A" => {
                    score += Moves::Rock as i32;
                }
                "B" => {
                    score += Moves::Paper as i32;
                }
                "C" => {
                    score += Moves::Scissors as i32;
                }
                _ => {}
            }
        }
        "Z" => {
            score += Results::Win as i32;
            match op_move {
                "A" => {
                    score += Moves::Paper as i32;
                }
                "B" => {
                    score += Moves::Scissors as i32;
                }
                "C" => {
                    score += Moves::Rock as i32;
                }
                _ => {}
            }
        }
        _ => {}
    }

    return score;
}


fn main() {
    let contents = fs::read_to_string("input2.txt").expect("Should have read the file");

    let lines = contents.lines();
    let mut tot_score_part1 = 0;
    let mut tot_score_part2 = 0;

    for line in lines {
        if line != "" {
            let split = line.split(" ");
            let moves = split.collect::<Vec<&str>>();

            let score = calculate_score_part1(moves[0], moves[1]);
            tot_score_part1 += score;
            let score = calculate_score_part2(moves[0], moves[1]);
            tot_score_part2 += score;
        }
    }
    println!("Total score part1: {}", tot_score_part1);
    println!("Total score part2: {}", tot_score_part2);

}
