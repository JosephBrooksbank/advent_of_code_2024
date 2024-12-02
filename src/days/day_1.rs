use crate::utils;
use crate::utils::file_helpers::{read_full_file, read_sample_file};

pub fn day_1_full() -> u32{
    let lines = read_full_file("day_1").unwrap();
    day_1(&lines)
}

pub fn day_1_sample() -> u32 {
    let lines = read_sample_file("day_1").unwrap();
    day_1(&lines)
}
fn day_1(lines: &Vec<String>) -> u32 {
    let (first_list, second_list) = get_lists(lines);

    let total_distance = get_total_distance(first_list, second_list);

    println!("{}", format!("Total Distance: {total_distance}"));
    total_distance
}

/// From a vector of lines, get two sorted lists of numbers
fn get_lists(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];

    for line in lines {
        let numbers : Vec<i32> = line.split("   ").map(|s| s.parse::<i32>().unwrap()).collect();
        first_list.push(numbers[0]);
        second_list.push(numbers[1]);
    }
    first_list.sort();
    second_list.sort();
    (first_list, second_list)
}

fn get_total_distance(first_list: Vec<i32>, second_list: Vec<i32>) -> u32 {
    let mut total_distance = 0;
    if (first_list.len() != second_list.len()) {
        panic!("List lengths do not match");
    }
    for (i, first_item) in first_list.iter().enumerate() {
        total_distance += get_distance(first_item, &second_list[i]);
    }
    total_distance
}

fn get_distance(a: &i32, b: &i32) -> u32 {
    b.abs_diff(*a)
}


#[cfg(test)]
mod tests {
    use crate::days::day_1::day_1_sample;

    #[test]
    fn sample_should_be_11() {
        let distance = day_1_sample();
        assert_eq!(distance, 11);
    }

}