use crate::utils::file_helpers::{read_full_file, read_sample_file};
use crate::utils::string_helpers::string_to_numbers;

enum Direction {
    Ascending,
    Descending
}

pub fn day_2_full() -> usize {
    let lines = read_full_file("day_2").unwrap();
    day_2(lines, true)
}

pub fn day_2_sample(part_2_enabled: bool) -> usize {
    let lines = read_sample_file("day_2").unwrap();
    day_2(lines, part_2_enabled)
}
fn day_2(lines: Vec<String>, part_2_enabled: bool) -> usize {
    let mut num_safe = 0;
    for line in lines {
        let mut numbers = string_to_numbers::<i64>(&line, " ");
        match check_numbers(&numbers) {
            Ok(_) => {
                num_safe+=1;
            }
            Err(index) => {
                if part_2_enabled {
                    for (i, _) in numbers.iter().enumerate() {
                        if try_again_with_removed_index(&numbers, i) {
                            num_safe += 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    num_safe
}

fn try_again_with_removed_index(numbers: &Vec<i64>, index_to_remove: usize) -> bool {
    let mut numbers = numbers.clone();
    numbers.remove(index_to_remove);
    match check_numbers(&numbers) {
        Ok(_) => {
            true
        },
        Err(index) => {
            false
        }
    }
}

fn check_numbers(numbers: &Vec<i64>) -> Result<(), usize> {
    let mut direction = Direction::Ascending;
    if numbers[0] - numbers[1] > 0 {
        direction = Direction::Descending;
    }
    for (i, x) in numbers.iter().enumerate() {
        if i == numbers.len() - 1 {
            break;
        }
        if !is_safe(x, &numbers[i+1], &direction) {
            return Err(i+1);
        }
    }
    Ok(())
}

fn is_safe(a: &i64, b: &i64, direction: &Direction) -> bool {
    if a == b || a.abs_diff(*b) > 3 {
        return false;
    }

    match direction {
        Direction::Ascending => {
            if a - b > 0 {
                return false;
            }
        },
        Direction::Descending => {
            if a - b < 0 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_2_part_1_sample_should_be_2() {
        assert_eq!(day_2_sample(false), 2);
    }
    #[test]
    fn day_2_part_2_sample_should_be_4() {
        assert_eq!(day_2_sample(true), 4);
    }
}
