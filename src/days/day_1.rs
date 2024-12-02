use crate::utils::file_helpers::{read_full_file, read_sample_file};

#[derive(Debug)]
pub struct Day1Data {
    total_distance: usize,
    similarity: usize,
}

pub fn day_1_full() -> Day1Data{
    let lines = read_full_file("day_1").unwrap();
    day_1(&lines)
}

pub fn day_1_sample() -> Day1Data {
    let lines = read_sample_file("day_1").unwrap();
    day_1(&lines)
}
fn day_1(lines: &Vec<String>) -> Day1Data {
    let (first_list, second_list) = get_lists(lines);

    let total_distance = get_total_distance(&first_list, &second_list);
    let similarity = get_similarity(&first_list, &second_list);

    Day1Data {
        total_distance,
        similarity,
    }
}

/// From a vector of lines, get lists of numbers
fn get_lists(lines: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut first_list: Vec<usize> = vec![];
    let mut second_list: Vec<usize> = vec![];

    for line in lines {
        let numbers : Vec<usize> = line.split("   ").map(|s| s.parse::<usize>().unwrap()).collect();
        first_list.push(numbers[0]);
        second_list.push(numbers[1]);
    }
    (first_list, second_list)
}

/// Clones and sorts a list
fn sort_list(list: &Vec<usize>) -> Vec<usize>{
    let mut list = list.clone();
    list.sort();
    list
}

fn get_total_distance(first_list: &Vec<usize>, second_list: &Vec<usize>) -> usize {

    if first_list.len() != second_list.len() {
        panic!("List lengths do not match");
    }

    let first_list = sort_list(first_list);
    let second_list = sort_list(second_list);

    let mut total_distance = 0;
    for (i, first_item) in first_list.iter().enumerate() {
        total_distance += get_distance(first_item, &second_list[i]);
    }
    total_distance
}

fn get_similarity(first_list: &Vec<usize>, second_list: &Vec<usize>) -> usize {
    let mut similarity = 0;
    for value in first_list {
        let count = second_list.into_iter().filter(|x| *x == value).collect::<Vec<&usize>>().len();
        similarity += value*count;
    }
    similarity
}

fn get_distance(a: &usize, b: &usize) -> usize {
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