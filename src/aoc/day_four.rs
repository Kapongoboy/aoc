use crate::aoc::*;

pub fn first_problem(path: &Path){
    let mut total_points = 0;
    let content: Vec<String> = get_file_content(path)
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    for line in content.iter() {
        if line.len() == 0 {continue};
        let split_line: Vec<String> = line.split('|')
            .map(|x| x.to_string())
            .collect()
            ;
        let winning_side_str = split_line[0].split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
            .split(' ')
            .filter(|item| !item.is_empty())
            .collect::<Vec<&str>>();
        let winning_side = winning_side_str.iter().map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        let player_side_str = split_line[1].trim()
            .split(' ')
            .filter(|item| !item.is_empty())
            .collect::<Vec<&str>>();
        let player_side = player_side_str.iter().map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        let mut points = 0;
        player_side.iter().for_each(|x| {
            if winning_side.contains(&x) {
                if points == 0 { points += 1;}
                else {points *= 2;}
            }
        });
        total_points += points;
    }
    println!("total points: {}", total_points);
}


pub fn second_problem(path: &Path){
    let content: Vec<String> = get_file_content(path)
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    let mut copies_vec: Vec<usize> = content.iter().map(|_| 1).collect();

    for (idx, line) in content.iter().enumerate() {
        if line.len() == 0 {continue};
        let split_line: Vec<String> = line.split('|')
            .map(|x| x.to_string())
            .collect();
        let winning_side_str = split_line[0].split(':')
            .collect::<Vec<&str>>()[1]
            .trim()
            .split(' ')
            .filter(|item| !item.is_empty())
            .collect::<Vec<&str>>();
        let winning_side = winning_side_str.iter().map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        let player_side_str = split_line[1].trim()
            .split(' ')
            .filter(|item| !item.is_empty())
            .collect::<Vec<&str>>();
        let player_side = player_side_str.iter().map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();
        let mut points = 0;
        player_side.iter().for_each(|x| {
            if winning_side.contains(&x) {
                points += 1;
            }
        });

        for _ in 0..copies_vec[idx]{
            (idx+1..idx+points+1).into_iter().for_each(|x| copies_vec[x] += 1);
        }
    }
    let total_copies: usize = copies_vec.iter().sum();
    println!("total copies: {}", total_copies-1);
}

