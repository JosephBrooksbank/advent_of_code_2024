use std::{fs, io};

pub fn read_file_line_by_line(file_path: &str) -> Result<Vec<String>, io::Error> {
    println!("Reading file: {}", file_path);
    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(contents) => {
            let lines = contents
                .trim_start_matches("\u{feff}")
                .lines()
                .map(|s| s.to_string())
                .collect();
            Ok(lines)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e)
        }
    }
}

pub fn read_sample_file(day: &str) -> Result<Vec<String>, io::Error> {
    let file_path = format!("src/days/input_files/{day}_sample.txt");
    return read_file_line_by_line(&file_path);
}

pub fn read_full_file(day: &str) -> Result<Vec<String>, io::Error> {
    let file_path = format!("src/days/input_files/{day}_full.txt");
    return read_file_line_by_line(&file_path);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_should_return_lines() {
        let lines = read_file_line_by_line("src/utils/test_file.txt").unwrap();
        assert_eq!(lines[0], "first line 2 3");
        assert_eq!(lines[1], "second line 4 6");
        assert_eq!(lines[2], "third 54 line &");
    }

    #[test]
    fn read_sample_file_should_return_lines() {
        let lines = read_sample_file("test").unwrap();
        assert_eq!(lines[0], "123 456");
    }
}