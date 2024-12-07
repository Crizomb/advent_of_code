use std::env;
use std::fs;
use std::path::Iter;

pub fn solve_day_1_2() {
    // --snip--
    let file_path = "/home/clement/RustroverProjects/advent_of_code/src/inputs/input_1.txt";
    println!("In file {file_path}");

    let (mut l_list, mut r_list) : (Vec<i32>, Vec<i32>) = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .split('\n')
        .map(|s|{
            let mut splited = s.split_whitespace();
            (
                splited.next().unwrap().parse::<i32>().unwrap(),
                splited.next().unwrap().parse::<i32>().unwrap()
            )
        }).unzip();

    l_list.sort();
}