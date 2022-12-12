use std::{fs, str::FromStr, alloc::System};
use anyhow::Result;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    pub fn execute(&self, stack: &mut [Vec<&str>]) { //-> &mut Vec<Vec<&str>> {
        println!("Count: {}, from: {}, to: {}", self.count, self.from, self.to);
        for _ in 0..self.count {
            let t = stack[self.from-1].get_mut(stack[self.from-1].len()-self.count..).unwrap();
            stack[self.to-1].extend_from_slice(t);
        }
    }
}


impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp = s.split_whitespace();
        let t2 = tmp.collect::<Vec<&str>>();
        return Ok(Move {
            count: t2[1].parse::<usize>().unwrap(),
            from: t2[3].parse()?,
            to: t2[5].parse()?,
        })
    }
}

fn part1(lines: std::str::Lines) {
    //let mut stacks = vec![vec!["G", "D", "V", "Z", "J", "S", "B"], vec!["Z", "S", "M", "G", "V", "P"], vec!["C", "L", "B", "S", "W", "T", "Q", "F"], vec!["H", "J", "G", "W", "M", "R", "V", "Q"], vec!["C", "L", "S", "N", "F", "M", "D"], vec!["R", "G", "C", "D"], vec!["H", "G", "T", "R", "J", "D", "S", "Q"], vec!["P", "F", "V"], vec!["D", "R", "S", "T", "J"]];
    let mut stacks = vec![vec!["N", "Z"], vec!["M", "C", "D"], vec!["P"]];

    //for stack in stacks {
    //    for item in stack {
    //        print!("{}", item);
    //    }
    //    println!();
    //}
   
    for line in lines {
        let m = line.parse::<Move>().unwrap();
        m.execute(stacks.as_mut_slice());
    }
    println!("--");
    for mut stack in stacks {
        print!("{}",stack.pop().unwrap());
    }

}

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("error");

    let lines = contents.lines();

    part1(lines.clone());

}

