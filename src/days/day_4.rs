use crate::days::day_4::Errors::OutOfBounds;
use crate::days::Day;
use std::cmp::max;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Day4;

enum Errors {
    OutOfBounds,
}

#[derive(EnumIter, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

struct Coords {
    x: usize,
    y: usize,
}
impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {

    fn new(grid: Vec<Vec<char>>) -> Self {
        Grid {grid}
    }
    fn get(&self, c: &Coords) -> char {
        self.grid[c.y][c.x]
    }

    fn get_next_coord(&self, s: &Coords, direction: &Direction) -> Result<Coords, Errors> {

        // assuming grid is uniform size
        let max_x = self.grid[0].len() - 1;
        let max_y = self.grid.len() - 1;

        match direction {
            Direction::Right => {
                if s.x == max_x {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x + 1, s.y))
                }
            }
            Direction::Left => {
                if s.x == 0 {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x - 1, s.y))
                }
            }
            Direction::Down => {
                if s.y == max_y {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x, s.y + 1))
                }
            }
            Direction::Up => {
                if s.y == 0 {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x, s.y - 1))
                }
            }
            Direction::DownLeft => {
                if s.x == 0 || s.y == max_y {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x - 1, s.y + 1))
                }
            }
            Direction::DownRight => {
                if s.x == max_x || s.y == max_y {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x + 1, s.y + 1))
                }
            }
            Direction::UpLeft => {
                if s.x == 0 || s.y == 0 {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x - 1, s.y - 1))
                }
            }
            Direction::UpRight => {
                if s.x == max_x || s.y == 0 {
                    Err(OutOfBounds)
                } else {
                    Ok(Coords::new(s.x + 1, s.y - 1))
                }
            }
        }
    }

    fn get_direction(&self, s: &Coords, direction: &Direction) -> Result<char, Errors> {
        let new_coord = self.get_next_coord(s, direction);
        match new_coord {
            Err(e) => Err(e),
            Ok(coords) => {
                Ok(self.get(&coords))
            }
        }
    }

    fn check_diagonal(&self, s: &Coords, up_dir: &Direction, down_dir: &Direction) -> bool {
        let upper =  match self.get_direction(s, up_dir) {
            Err(_) => return false,
            Ok(c) => c
        };
        let lower = match self.get_direction(s, down_dir) {
            Err(_) => return false,
            Ok(c) => c
        };

        if (upper == 'M' && lower == 'S') {
            return true;
        }
        if (upper == 'S' && lower == 'M') {
            return true;
        }
        false
    }

    fn check_dmas(&self, s: &Coords) -> bool {
        let mut total = 0;
        if (self.check_diagonal(s, &Direction::UpRight, &Direction::DownLeft)) {
            println!("{} {} had a diagonal from top right", s.x, s.y);
            total +=1;
        }
        if (self.check_diagonal(s, &Direction::UpLeft, &Direction::DownRight)) {
            println!("{} {} had a diagonal from top left", s.x, s.y);
            total += 1;
        }
        total == 2
    }
    fn check_xmas(&self, s: &Coords, direction: &Direction, letter_index: usize) -> bool {

        let letters: Vec<char> = vec!['M', 'A', 'S'];
        // assuming grid is uniform size
        let new_coords = self.get_next_coord(s, direction);

        match new_coords {
            Err(_) => {
                // println!("could not find the next letter");
                return false
            },
            Ok(coords) => {
                let compare_value = self.get(&coords);
                if compare_value == letters[letter_index] {
                    if letter_index == 2 {
                        // println!("S at {}, {}", coords.x, coords.y);
                        return true;
                    }
                    // print!("{} ", letters[letter_index]);
                    return self.check_xmas(&coords, direction, letter_index + 1);
                }
            }
        }

        // println!("could not find the next letter");
        false
    }
}

#[derive(Debug)]
pub struct Day4Data {
    part1: usize,
    part2: usize,
}
impl Day4 {
    pub fn find_xmas(&self, grid: Grid) -> Day4Data {
        let mut total_count = 0;
        let mut total_dmas = 0;
        for (y, row) in grid.grid.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == 'X' {
                    for direction in Direction::iter() {
                        // println!("{:#?}", direction);
                        // print!("X ");
                        if grid.check_xmas(&Coords::new(x, y), &direction, 0) {
                            total_count += 1;
                        } else {}
                    }
                }
                if *val == 'A' {
                    if grid.check_dmas(&Coords::new(x,y)) {
                        total_dmas += 1;
                    }
                }
            }
        }
        Day4Data {
            part1: total_count,
            part2: total_dmas,
        }
    }
}

impl Day<Day4Data> for Day4 {
    fn id(&self) -> &'static str {
        "day_4"
    }

    fn run(&self, lines: Vec<String>) -> Day4Data {
        let grid: Grid = Grid::new(lines.iter().map(|s| s.chars().collect()).collect());
        self.find_xmas(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_should_be_18() {
        let day4 = Day4;
        assert_eq!(day4.run_sample().part1, 18);
    }
    #[test]
    fn part_2_sample_should_be_9() {
        let day4 = Day4;
        assert_eq!(day4.run_sample().part2, 9);
    }
}
