use std::{collections::BTreeMap, fs::{ self}, str::FromStr, num::ParseIntError, thread::available_parallelism, sync::Arc};
use anyhow::anyhow;


#[derive(Eq, Debug, Clone, Copy)]
enum RuleRange{
    Rule(i64, i64),
    Partial(i64)
}

impl PartialOrd for RuleRange{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmp(other).into()
    }
}

impl PartialEq for RuleRange {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuleRange::Rule( start, range), RuleRange::Partial(search)) 
            | (RuleRange::Partial(search), RuleRange::Rule( start, range)) => !(search < start) && !(start > &((search + range))),
            (RuleRange::Partial(a), RuleRange::Partial(b)) => a == b,
            (RuleRange::Rule( start1, range1), RuleRange::Rule(start2, range2)) => start1 == start2 && range1 == range2,
        }
    }
}

impl Ord for RuleRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (RuleRange::Rule(start, range), RuleRange::Partial(search)) | (RuleRange::Partial(search), RuleRange::Rule(start, range)) => {
                return if search < start {
                    std::cmp::Ordering::Less
                } else if search >= &(start + range) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            },
            (RuleRange::Partial(a), RuleRange::Partial(b)) => a.cmp(b),
            (RuleRange::Rule(start1, range1), RuleRange::Rule(start2, range2)) => 
                if start1 == start2 {
                    range1.cmp(range2)
                } else {
                    start1.cmp(start2)
                }
            ,
        }
    }
}

type Rules = Vec<BTreeMap<RuleRange, i64>>;

struct Almanach {
    rules: Arc<Rules>,
    seeds: Vec<i64>,
}

impl Almanach {
    fn new(rules: Rules, seeds: Vec<i64>) -> Self {
        Self {rules:  Arc::new(rules), seeds }
    }

    fn from_file(path: &str) -> anyhow::Result<Self> {
        fs::read_to_string(path)?.parse()
    }

    fn run_one(&self) -> Result<i64, anyhow::Error> {
        self.seeds.iter()
            .map(|seed| Self::location_from_seed(&self.rules, *seed))
            .min()
            .ok_or(anyhow::anyhow!("No min found"))
    }

    fn location_from_seed(rules: &Rules, seed: i64) -> i64 {
        rules.iter().fold(seed, |acc, rule_set| {
            let search_rule = RuleRange::Partial(acc);
            let rule = rule_set.get_key_value(&search_rule);
            match rule {
                Some((RuleRange::Rule(start, _),start_dest)) => start_dest + (acc - start),
                None => {
                    acc
                },
                _ => panic!("Error getting rule"),
            }
        })
    }

    fn get_seed_ranges(&self) -> Vec<RuleRange> {
        let mut seed_ranges: Vec<RuleRange> = Vec::new();
        let mut index = 0;
        while index < self.seeds.len() {
            seed_ranges.push(RuleRange::Rule(self.seeds[index], self.seeds[index + 1]));
            index = index + 2;
        };
        seed_ranges
    }

    fn run_two(&self) -> Result<i64, anyhow::Error> {
        self.get_seed_ranges().into_iter().map(|rule_range| {
            let RuleRange::Rule(seed_start, seed_range) = rule_range else { panic!("Error getting seed range") };

            let num_threads = available_parallelism().expect("Cannot get number of available_parallelism").get();
            let mut threads = Vec::new();

            for n in 0..num_threads {
                let (start, stop) = ((seed_range / num_threads as i64) * n as i64 + seed_start, (seed_range / num_threads as i64) * (n + 1) as i64 + seed_start);
                let rules = self.rules.clone();
                threads.push(std::thread::spawn(move || {
                    (start..stop).map(|i| Self::location_from_seed(&rules, i)
                    ).min().ok_or(anyhow::anyhow!("No min found"))  
                }))
            }

            threads
                .into_iter()
                .map(|t| t.join().expect("Thread failed"))
                .collect::<Result<Vec<i64>, anyhow::Error>>()
                .and_then(|v| v.iter()
                    .min()
                    .ok_or(anyhow!("No min found"))
                    .map(|i| *i)
                )
        
        }).collect::<Result<Vec<i64>, anyhow::Error>>().and_then(|v| v.iter().min().ok_or(anyhow!("No min found")).map(|i| *i))
    }
}

impl FromStr for Almanach {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules_sets: Vec<&str> = s.split("\n\n").collect();
        let seeds: Vec<i64> = rules_sets[0].split(":").nth(1).expect("Error parsing seed line").split_whitespace().map(|e| e.parse::<i64>()).collect::<Result<Vec<i64>, ParseIntError>>()?;

        let rules: Rules = rules_sets.into_iter().skip(1).map(|rule_set| -> Result<BTreeMap<RuleRange, i64>, anyhow::Error> {
            let mut rule_map: BTreeMap<RuleRange, i64> = BTreeMap::new();
            let rule_lines: Vec<&str> = rule_set.trim().split("\n").skip(1).collect();
            for rule_line in rule_lines {
                let mut rule_parts = rule_line.split_whitespace();
                let rule_dest_start: i64 = rule_parts.next().expect(format!("Error parsing rule dest start: '{}'", rule_line).as_str()).parse()?;
                let rule_src_start: i64 = rule_parts.next().expect(format!("Error parsing rule src start: '{}'", rule_line).as_str()).parse()?;
                let rule_src_range: i64 = rule_parts.next().expect(format!("Error parsing rule src range: '{}'", rule_line).as_str()).parse()?;
                rule_map.insert(RuleRange::Rule(rule_src_start, rule_src_range), rule_dest_start);
            }
            Ok(rule_map)
        }).collect::<Result<Rules, anyhow::Error>>()?;

        Ok(Self::new(rules, seeds))

    }
}

pub fn part_one(path: &str) -> Result<i64, anyhow::Error> {
    let part_one = Almanach::from_file(path)?;
    part_one.run_one()
}

pub fn part_two(path: &str) -> Result<i64, anyhow::Error> {
    let part_one = Almanach::from_file(path)?;
    part_one.run_two()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::RuleRange;

    #[test]
    fn test_partial_range_is_equal_to_rule_range() {
        let rule = RuleRange::Rule(1, 3);
        let partial = RuleRange::Partial(2);
        assert_eq!(rule, partial);
    }

    #[test]
    fn test_partial_range_is_less_than_rule_range() {
        let rule = RuleRange::Rule(1, 3);
        let partial = RuleRange::Partial(0);
        assert!(partial < rule);
    }

    #[test]
    fn test_partial_range_is_greater_than_rule_range() {
        let rule = RuleRange::Rule(1, 3);
        let partial = RuleRange::Partial(5);
        assert!(partial > rule);
    }

    #[test]
    fn test_partial_range_is_greater_than_rule_edge() {
        let rule = RuleRange::Rule(1, 3);
        let partial = RuleRange::Partial(4);
        assert!(partial > rule);
    }

    #[test]
    fn test_partial_range_is_equal_to_rule_top_edge() {
        let rule = RuleRange::Rule(98, 2);
        let partial = RuleRange::Partial(99);
        assert_eq!(rule, partial);
    }

    #[test]
    fn test_partial_range_is_equal_to_rule_bottom_edge() {
        let rule = RuleRange::Rule(98, 2);
        let partial = RuleRange::Partial(98);
        assert_eq!(rule, partial);
    }

    #[test]
    fn test_partial_range_is_less_than_rule_bottom_edge() {
        let rule = RuleRange::Rule(98, 2);
        let partial = RuleRange::Partial(97);
        assert!(partial < rule);
    }

    #[test]
    fn test_two_different_rule_ranges_are_not_equal() {
        let rule = RuleRange::Rule(1, 3);
        let rule2 = RuleRange::Rule(5, 3);
        assert_ne!(rule, rule2);
    }

    #[test]
    fn test_rule_map_gets_correct_rule_range() {
        let mut rule_map = BTreeMap::new();
        let rule_range = RuleRange::Rule(1, 3);
        let rule_dest = 2;  
        rule_map.insert(rule_range, rule_dest);
        let search_rule = RuleRange::Partial(2);
        let rule = rule_map.get_key_value(&search_rule);
        assert_eq!(rule, Some((&RuleRange::Rule(1, 3), &2)));
    }

    #[test]
    fn test_insert_multiple_rule_ranges_into_map() {
        let mut rule_map = BTreeMap::new();
        let rule_range = RuleRange::Rule(1, 3);
        let rule_dest = 2;
        let rule_range2 = RuleRange::Rule(5, 3);
        let rule_dest2 = 6;
        rule_map.insert(rule_range, rule_dest);
        rule_map.insert(rule_range2, rule_dest2);
        dbg!(&rule_map);
        let search_rule = RuleRange::Partial(2);
        let rule = rule_map.get_key_value(&search_rule);
        assert_eq!(rule, Some((&RuleRange::Rule(1, 3), &2)));
    }

    #[test]
    fn test_part_one_from_file() {
        let part_one = super::Almanach::from_file("./day5_test.txt").expect("Error parsing file");
        assert_eq!(part_one.run_one().expect("Error running part one"), 35);
    }

    #[test]
    fn test_part_two_from_file() {
        let part_one = super::Almanach::from_file("./day5_test.txt").expect("Error parsing file");
        assert_eq!(part_one.run_two().expect("Error running part two"), 46);
    }
}