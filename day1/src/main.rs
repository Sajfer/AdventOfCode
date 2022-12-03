use std::fs;


pub fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have read the file");

    let list = contents.lines();
    let mut sums=Vec::new();
    let mut sum = 0;
    for line in list {
        match line.parse::<i32>() {
            Ok(n) => sum += n,
            Err(e) => {
                sums.push(sum);
                sum = 0;
            }
        }
    }
    sums.sort();
    let top_three = sums[sums.len()-1] + sums[sums.len()-2] + sums[sums.len()-3];
    println!("highest value: {}", top_three);

}
