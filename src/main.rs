use advent_of_code_2024::days::{*};
use advent_of_code_2024::days::day::Day;

mod utils;

fn main() {
    let day_1_solution = day_1::day_1_full();
    println!("{}", format!("Day 1 Distance: {:#?}", day_1_solution ));

    let day_2_solution = day_2::day_2_full();
    println!("Day 2 solution is {day_2_solution}");

    let day3 = day_3::Day3;
    day3.run_full();

    let day4 = day_4::Day4;
    day4.run_full();

}
