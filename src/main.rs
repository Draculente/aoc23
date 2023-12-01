use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use day1::part2;

mod day1;

fn main() {
    let input = read_file("./day1.txt").expect("reading file went wrong");

    if let Ok(res) = part2(input) {
        println!("The result is: {}", res);
    }
}

fn read_file(path: &str) -> Result<Vec<String>, anyhow::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}
