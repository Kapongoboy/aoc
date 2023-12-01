pub mod day_one;
pub mod day_two;

use std::{fs::File, path::Path};
use std::io::Read;

pub struct Config<'a> {
    day: i32,
    problem: i32,
    path: &'a Path,
}

impl<'a> Config<'a>{
    pub fn new(day: i32, problem: i32, path: &String) -> Config{
        Config {
            day,
            problem,
            path: Path::new(path),
        }
    }
}

pub fn get_file_content(path: &Path) -> String {
    let mut file = File::open(path).expect("The file could not be opened");
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return contents,
        Err(_) => {
            eprintln!("error encountered opening the file, returning empty string");
            return String::new();
        }
    };

}

pub fn get_numbers(s: &String) -> Vec<String> {
    let s_vector: Vec<&str> = s.split("\n").collect();
    let mut nums: Vec<String> = Vec::new();
    for item in s_vector.iter(){
        let mut line_nums = String::from("");
        for c in item.chars() {
            if c.is_numeric() {
                line_nums.push(c);
            }
        }
        if !line_nums.is_empty() {
            let mut clean_line = String::with_capacity(2);
            if line_nums.len() == 1 {
                clean_line.insert(0, line_nums.as_bytes()[0] as char);
                clean_line.insert(1, line_nums.as_bytes()[0] as char);
            } else {
                clean_line.insert(0, line_nums.as_bytes()[0] as char);
                clean_line.insert(1, line_nums.as_bytes()[line_nums.len() - 1] as char);
            }
            nums.push(clean_line)
        };
    }
    return nums;
}

pub fn add_string_numbers(s: &Vec<String>) -> usize {
    let sum_values: usize = s.into_iter().map(|x| x.parse::<usize>().expect("The value {} is not a number")).sum();
    return sum_values;
}

pub fn run(instructions: Config){
    if instructions.day == 1 {
        if instructions.problem == 1 {
            crate::aoc::day_one::first_problem(instructions.path);
        }
    } else {
        println!("Either this day wasn't completed or doesn't exist sorry");
    }
}

#[cfg(test)]
mod tests{
    use crate::aoc::*;

    #[test]
    fn file_read(){
        let text = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        let path = Path::new("./data/day1_example.txt");
        let result = get_file_content(&path);
        assert_eq!(text, result);
    }

    #[test]
    fn number_parsing(){
        let path = Path::new("./data/day1_example.txt");
        let contents = get_file_content(&path);
        let result = get_numbers(&contents);
        assert_eq!(vec!["12", "38", "15", "77"], result);
    }

    #[test]
    fn number_adding(){
        let path = Path::new("./data/day1_example.txt");
        let contents = get_file_content(&path);
        let numbers = get_numbers(&contents);
        let result = add_string_numbers(&numbers);
        assert_eq!(result, 142);
    }
}
