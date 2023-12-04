use crate::aoc::*;

pub fn first_problem(path: &Path){
    let content: Vec<String> = get_file_content(path)
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    content.iter().for_each(|x| println!("line:\n{}\n)", x));
}

pub fn second_problem(path: &Path){
    let content: Vec<String> = get_file_content(path)
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    content.iter().for_each(|x| println!("line:\n{}\n)", x));
}
