pub mod aoc;
use std::env;
use aoc::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    aoc::run(parse_args(&args));
}

fn parse_args(args: &[String]) -> Config {
    if args.len() < 4 {
        panic!("not enough arguments");
    }
    let (day, number) = (
        args[1].parse::<i32>().expect("Expected a number for the day"),
        args[2].parse::<i32>().expect("Expected a number for the problem")
    );
    let path_str = &args[3];
    let config = Config::new(day, number, path_str);
    return config;
}
