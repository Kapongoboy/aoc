use crate::aoc::*;

pub fn first_problem(path: &Path){
    let contents = get_file_content(&path);
    let numbers = get_numbers(&contents);
    let result = add_string_numbers(&numbers);
    println!("The resulting sum is: {}", result);
}
