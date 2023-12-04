use crate::aoc::*;

pub fn first_problem(path: &Path){
    let mut total_points = 0;
    let content: Vec<String> = get_file_content(path)
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    // content.iter().for_each(|x| println!("\n{}\n", x));
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
    let mut total_copies = 0;
    let mut copies = 1;
    let content: Vec<String> = get_file_content(path)
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    // content.iter().for_each(|x| println!("\n{}\n", x));
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
        total_copies += copies;
    }
    println!("total scratch cards: {}", total_copies);
}
