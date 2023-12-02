use crate::aoc::*;
use std::collections::HashMap;
use std::cmp;

pub fn first_problem(path: &Path) {
    let contents = get_file_content(&path);
    println!("contents: \n{}", contents);
    let lines: Vec<&str> = contents.split("\n").collect();
    let limits: HashMap<u32, &str> = HashMap::from([(12, "d"), (13, "n"), (14, "e")]);
    let mut validity: Vec<bool> = Vec::new();
    let mut sum = 0;
    for (idx, line) in lines.into_iter().enumerate() {
        validity.push(true);
        if line.is_empty() {continue;};
        let no_white = line.replace(" ", "");
        let g_split = no_white.split(':').collect::<Vec<&str>>();
        let game = g_split[0];
        let results = g_split[1];
        for set in results.split(';').into_iter(){
            let cubes: Vec<&str> = set.split(',').collect();
            for cube in cubes.into_iter() {
                let num = cube.matches(char::is_numeric).collect::<Vec<&str>>().join("").parse::<u32>().unwrap();
                let color = cube.chars().last().unwrap().to_string();
                let v: Vec<Option<bool>> = limits.iter().map(|(key, val)| {
                    println!("idx {}: key {} num {}, val {} color {}", idx, key, &num, *val, color.as_str());
                    if (key < &num) && (*val == color.as_str()) {
                        Some(false)
                    } else {
                        None
                    }
                }).collect();
                println!("v: {:?}", v);
                if v.iter().any(|&x| x == Some(false)) {
                    validity[idx] = false;
                    break;
                }
            }
        }
        if validity[idx] == true {
            sum += game.matches(char::is_numeric).collect::<Vec<&str>>().join("").parse::<u32>().unwrap();        
        }
    }
    println!("sum: {}", sum);
}

pub fn second_problem(path: &Path) {
    let contents = get_file_content(&path);
    println!("contents: \n{}", contents);
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut validity: Vec<bool> = Vec::new();
    let mut sum: u32 = 0;
    for line in lines.into_iter() {
        let mut m: HashMap<char, u32 >= HashMap::from([('e', 0), ('d', 0), ('n', 0)]);
        validity.push(true);
        if line.is_empty() {continue;};
        let no_white = line.replace(" ", "");
        let g_split = no_white.split(':').collect::<Vec<&str>>();
        let results = g_split[1];
        for set in results.split(';').into_iter(){
            let cubes: Vec<&str> = set.split(',').collect();
            for cube in cubes.into_iter() {
                let num = cube.matches(char::is_numeric).collect::<Vec<&str>>().join("").parse::<u32>().unwrap();
                let color = cube.chars().last().unwrap();
                m.entry(color).and_modify(|value| *value = cmp::max(*value,  num)).or_insert(0);
            }
        }
        sum += m.values().product::<u32>();
    }
    println!("sum: {}", sum);
}

