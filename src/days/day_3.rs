use regex::Regex;
use crate::days::day::Day;

pub struct Day3;

impl Day3 {
    fn get_numbers(line: &String, enabled: &mut bool) -> Vec<(usize, usize)> {
        // a bit of a hack-- rust regex expects each match to have the same number of captures. So I'm padding the
        // "do" and "don't" groups to have 3 groups, just like the mul group.
        let re = Regex::new(r"(mul\((?<first>\d+),(?<second>\d+)\)|((do\(\)))|((don't\(\))))").unwrap();
        let mut values = vec![];
        for (_, [full, left, right]) in re.captures_iter(line).map(|cap| cap.extract()) {
            if *enabled && full.starts_with("m") {
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                values.push((left, right));
            }
            if full == "don't()" {
                *enabled = false;
            }
            if full == "do()" {
                *enabled = true;
            }
        }
        values
    }

}
impl Day<usize> for Day3 {
    fn id(&self) -> &'static str {
        "day_3"
    }

    fn run(&self, lines: Vec<String>) -> usize {
        let mut total = 0;
        let mut enabled = true;
        for line in lines {
            let numbers = Self::get_numbers(&line, &mut enabled);
            for pair in numbers {
                total += pair.0 * pair.1
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_sample_should_be_161() {
        let day3 = Day3;
        assert_eq!(day3.run_sample(), 48)
    }
}
