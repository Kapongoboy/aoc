use crate::aoc::*;

pub fn first_problem(path: &Path){
    let content = get_file_content(path);
    let coors = [(-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)];
    let mut sum: usize = 0;
    println!("content: \n{}", content);
    for line in content.split("\n").collect::<Vec<&str>>().into_iter(){
        let values: Vec<(usize, char)> = line.char_indices().collect();
        for (idx, c) in values.iter(){
            if *c == '.' {
                continue;
            }
            if c.is_digit(10){
                coors.iter().map(|x| {
                    let adj = 
                });
            }
        }
    }
}

pub fn second_problem(path: &Path){
    let content = get_file_content(path);
    println!("content: \n{}", content);
}
