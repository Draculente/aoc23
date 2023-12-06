use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// use day1::part2;
use day5::part_two;

mod day1;
mod day5;
mod day6;

fn main() {
    // let input = read_file("./day1.txt").expect("reading file went wrong");

    // if let Ok(res) = part_one("./day5.txt") {
    //     println!("The result is: {}", res);
    // } 

    use std::time::Instant;
    let now = Instant::now();

    match part_two("./day5.txt") {
        Ok(res) => println!("The result is: {}", res),
        Err(e) => println!("Error: {}", e),
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn read_file(path: &str) -> Result<Vec<String>, anyhow::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}
