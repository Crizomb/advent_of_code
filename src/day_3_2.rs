use regex::Regex;
use std::fs::read_to_string;

pub fn solve_day_3_2() {
    let file_path = "/home/clement/RustroverProjects/advent_of_code/src/inputs/input_3.txt";
    println!("In file {file_path}");

    let content_string = read_to_string(file_path).expect("Something went wrong reading the file");
    let disable_regex = Regex::new(r"don't\(\)[\s\S]*?do\(\)").unwrap();
    let content_with_disable = disable_regex.replace_all(&content_string, "");
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = mul_regex.captures_iter(&content_with_disable);
    let get_int = caps
        .map(|x| x[1].parse::<i32>().unwrap() * x[2].parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{get_int}");
}