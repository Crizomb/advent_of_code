use std::env;
use std::fs;
use std::path::Iter;
use std::collections::HashMap;

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

    // let mut similarity = 0;
    let mut r_count_map : HashMap<i32, i32> = r_list.iter()
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;
            acc
        });

    let similarity = l_list.iter().fold(0, |mut acc, x| {
        acc += x * *r_count_map.entry(*x).or_insert(0);
        acc
    });

    println!("{similarity}");

}