use std::{num::ParseIntError, str::FromStr};

use anyhow::anyhow;

#[derive(Debug)]
struct Race {
    duration: i64,
    record_distance: i64,
}

impl Race {
    fn new(duration: i64, record_distance: i64) -> Self {
        Self {
            duration,
            record_distance,
        }
    }

    fn get_options(&self) -> Vec<i64> {
        let mut options = Vec::new();
        for i in 0..self.duration {
            options.push(i * (self.duration - i));
        }
        options
    }

    fn get_number_of_options_to_win(&self) -> i64 {
        self.get_options()
            .iter()
            .filter(|d| **d > self.record_distance)
            .collect::<Vec<&i64>>()
            .len() as i64
    }
}

struct Day6 {
    races: Vec<Race>,
}

impl Day6 {
    fn new(races: Vec<Race>) -> Self {
        Self { races }
    }

    fn run_one(&self) -> anyhow::Result<i64> {
        Ok(self
            .races
            .iter()
            .map(|r| r.get_number_of_options_to_win())
            .product::<i64>())
    }

    fn from_file(path: &str) -> anyhow::Result<Self> {
        let input = std::fs::read_to_string(path)?;
        input.parse::<Self>()
    }
}

impl FromStr for Day6 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        // dbg!(&lines.clone().collect::<Vec<&str>>());

        let times = lines
            .get(0)
            .ok_or(anyhow!("Error parsing times line"))?
            .split(":")
            .nth(1)
            .ok_or(anyhow!("Error parsing times line"))?
            .trim()
            .split_whitespace()
            .map(|e| e.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>()?;

        let distances = lines
            .get(1)
            .ok_or(anyhow!("Error getting distances line"))?
            .split(":")
            .nth(1)
            .ok_or(anyhow!("Error splitting distances line"))?
            .trim()
            .split_whitespace()
            .map(|e| e.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>()?;

        let races = times
            .into_iter()
            .enumerate()
            .map(|(i, time)| -> anyhow::Result<Race> {
                let distance = distances.get(i).ok_or(anyhow!("Error getting distance"))?;
                Ok(Race::new(time, *distance))
            })
            .collect::<anyhow::Result<Vec<Race>>>()?;

        Ok(Day6::new(races))
    }
}

pub fn part_one(path: &str) -> anyhow::Result<i64> {
    let day6 = Day6::from_file(path)?;
    day6.run_one()
}

#[cfg(test)]
mod tests {
    use crate::day6::Day6;

    use super::Race;

    #[test]
    fn test_race_has_correct_amount_of_options() {
        let race = Race::new(7, 9);
        assert_eq!(race.get_number_of_options_to_win(), 4);
    }

    #[test]
    fn test_race_has_correct_amount_of_options_2() {
        let race = Race::new(15, 40);
        assert_eq!(race.get_number_of_options_to_win(), 8);
    }

    #[test]
    fn test_race_has_correct_amount_of_options_3() {
        let race = Race::new(30, 200);
        assert_eq!(race.get_number_of_options_to_win(), 9);
    }

    #[test]
    fn get_solution_for_part_two() {
        let race = Race::new(61709066, 643118413621041);
        assert_eq!(race.get_number_of_options_to_win(), 9);
    }

    #[test]
    fn test_day6_part_one_test_data() {
        let day6 = Day6::from_file("./day6_test.txt").expect("Error parsing day6");
        assert_eq!(day6.run_one().expect("Error running day6"), 288);
    }
}
