pub fn part1(input: Vec<String>) -> Result<i32, anyhow::Error> {
    let res = input.into_iter().map(handle_part1_line).sum::<i32>();
    Ok(res)
}

fn handle_part1_line(line: String) -> i32 {
    let mut first: i32 = -1;
    let mut second: i32 = -1;

    for c in line.chars() {
        if !c.is_numeric() {
            continue;
        }
        let digit = c.to_digit(10).expect("not a number") as i32;
        if first < 0 {
            first = digit;
        }
        second = digit;
    }

    first * 10 + second
}

pub fn part2(input: Vec<String>) -> Result<i32, anyhow::Error> {
    let res = input.into_iter().map(handle_part2_line).sum::<i32>();
    Ok(res)
}

fn handle_part2_line(line: String) -> i32 {
    let mut first: i32 = -1;
    let mut second: i32 = -1;

    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            let digit = c.to_digit(10).expect("not a number") as i32;
            if first < 0 {
                first = digit;
            }
            second = digit;
        } else {
            if let Some(substring) = line.get(i..) {
                let str_digit = digits
                    .iter()
                    .enumerate()
                    .find(|(_, d)| substring.starts_with(**d));
                if let Some((index, _)) = str_digit {
                    if first < 0 {
                        first = (index + 1) as i32;
                    }
                    second = (index + 1) as i32;
                }
            }
        }
    }

    first * 10 + second
}

#[cfg(test)]
mod tests {
    use crate::day1::{handle_part1_line, handle_part2_line, part1, part2};

    #[test]
    fn line1_with_only_one_digit_is_correct() {
        let line = "treb7uchet".to_owned();
        assert_eq!(handle_part1_line(line), 77)
    }

    #[test]
    fn line1_with_exactly_two_digits_is_correct() {
        let line = "1abc2".to_owned();
        assert_eq!(handle_part1_line(line), 12);
    }

    #[test]
    fn line1_with_more_than_two_digits_is_correct() {
        let line = "a1b2c3d4e5f".to_owned();
        assert_eq!(handle_part1_line(line), 15)
    }

    #[test]
    fn aoc1_test_input_is_correct() {
        let file = "1abc2\n\
pqr3stu8vwx\n\
a1b2c3d4e5f\n\
treb7uchet"
            .to_owned();

        let res = part1(file.lines().map(|l| l.to_owned()).collect()).expect("Error running task");
        assert_eq!(res, 142)
    }

    #[test]
    fn line2_with_exactly_two_digits_is_correct() {
        let line = "7pqrstsixteen".to_owned();
        assert_eq!(handle_part2_line(line), 76)
    }

    #[test]
    fn line2_with_three_digits_is_correct() {
        let line = "eightwothree".to_owned();
        assert_eq!(handle_part2_line(line), 83)
    }

    #[test]
    fn line2_with_one_digit_is_correct() {
        let line = "two".to_owned();
        assert_eq!(handle_part2_line(line), 22)
    }

    #[test]
    fn aoc2_test_input_is_correct() {
        let file = "two1nine\n\
eightwothree\n\
abcone2threexyz\n\
xtwone3four\n\
4nineeightseven2\n\
zoneight234\n\
7pqrstsixteen"
            .to_owned();

        let res = part2(file.lines().map(|l| l.to_owned()).collect()).expect("Error running task");
        assert_eq!(res, 281)
    }
}
