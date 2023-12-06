use std::{str::FromStr, num::ParseIntError};

use anyhow::anyhow;

struct Race {
    duration: i32,
    record_distance: i32,
}

impl Race {
    fn new(duration: i32, record_distance: i32) -> Self {
        Self {
            duration,
            record_distance,
        }
    }

    fn get_options(&self) -> Vec<i32> {
        let mut options = Vec::new();
        for i in 0..self.duration {
            options.push(i * (self.duration - i));
        }
        options
    }

    fn get_number_of_options(&self) -> i32 {
        self.get_options().iter().filter(|d| **d >= self.record_distance).collect::<Vec<&i32>>().len() as i32
    }
}

struct Day6 {
    races: Vec<Race>
}

impl Day6 {
    fn new(races: Vec<Race>) -> Self {
        Self {
            races,
        }
    }

    fn run_one(&self) -> anyhow::Result<i32> {
        Ok(self.races.iter().map(|r| r.get_number_of_options()).product::<i32>())
    }

    fn from_file(path: &str) -> anyhow::Result<Self> {
        let input = std::fs::read_to_string(path)?;
        input.parse::<Self>()
    }
}

impl FromStr for Day6 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();

        dbg!(&lines.clone().collect::<Vec<&str>>());
        
        // let times = lines.nth(0).ok_or(anyhow!("Error parsing times line"))?.split(":").nth(1).ok_or(anyhow!("Error parsing times line"))?.trim().split_whitespace().map(|e| e.parse::<i32>()).collect::<Result<Vec<i32>, ParseIntError>>()?;

        // let distances = lines.nth(1).ok_or(anyhow!("Error getting distances line"))?.split(":").nth(1).ok_or(anyhow!("Error splitting distances line"))?.trim().split_whitespace().map(|e| e.parse::<i32>()).collect::<Result<Vec<i32>, ParseIntError>>()?;

        let distance_and_time: Vec<Vec<i32>> = lines.map(|line| -> Result<Vec<i32>, anyhow::Error> {
            let distance = line.split(":").skip(1).flat_map(|p| p.split_whitespace().into_iter()).map(|n| n.parse::<i32>()).collect::<Result<Vec<i32>, ParseIntError>>()?;
            Ok(distance)
        }).collect::<Result<Vec<Vec<i32>>, anyhow::Error>>()?;

        

        Ok(Day6::new(races))
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::Day6;

    #[test]
    fn test_day6_part_one() {
        let day6 = Day6::from_file("./day6_test.txt").expect("Error parsing day6");
        assert_eq!(day6.run_one().expect("Error running day6"), 288);
    }
}