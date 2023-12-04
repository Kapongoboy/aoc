use crate::aoc::*;

pub fn first_problem(path: &Path){
    let content: Vec<String> = get_file_content(path)
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let coors: Vec<(isize, isize)> = Vec::from([(-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)]);
    let mut valid: Vec<String> = Vec::new();
    let mut schem = false;

    for (y, line) in content.iter().enumerate() {
        let mut tmp: String = String::from("");
        for (x, c) in line.chars().enumerate() {
            if c == '.'{
                continue;
            } else {
                if c.is_digit(10){
                    tmp.push(c);
                    for coor in coors.iter(){
                        let n_ydx = y as isize + coor.0;
                        let n_xdx = x as isize + coor.1;
                        let line_result: Option<&String> = content.get(n_ydx as usize);
                        match line_result {
                            Some(v) => {
                                let result_vector = v.clone()
                                    .chars()
                                    .collect::<Vec<char>>();
                                let result = result_vector.get(n_xdx as usize);
                                match result {
                                    Some(v) => {
                                        if (*v != '.') && (!v.is_digit(10)){
                                            schem = true;
                                            break;
                                        }
                                    },
                                    None => continue
                                };
                            },
                            None => continue
                        };
                    }
                }
            }
            if x != line.len() - 1{
                let idx = x as isize + 1;
                if !line.chars().collect::<Vec<char>>()[idx as usize].is_digit(10){
                    if schem {
                        valid.push(tmp.clone());
                    }
                    tmp = "".to_string();
                    schem = false;
                }
            } else {
                if schem{
                    valid.push(tmp.clone());
                    schem = false;
                }
            }
        }
    }

    let sum: usize = valid.iter()
        .map(|x| x.parse::<usize>().unwrap())
        .sum();

    println!("final sum: {}", sum);
}

pub fn second_problem(path: &Path){
    let content = get_file_content(path);
    println!("content: \n{}", content);
}
