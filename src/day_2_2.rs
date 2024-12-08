use std::fs;

pub fn is_safe(level: &Vec<i32>, skip_i: usize) -> bool {
    let level_without_skip_i = level
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != skip_i)
        .map(|(_, x)| *x)
        .collect::<Vec<i32>>();

    let should_increase = level_without_skip_i[1] > level_without_skip_i[0];

    level_without_skip_i.windows(2).all(|win| {
        (win[1] - win[0]).abs() <= 3 && (win[1] > win[0]) == should_increase && win[1] != win[0]
    })
}
pub fn solve_day_2_2() {
    let file_path = "/home/clement/RustroverProjects/advent_of_code/src/inputs/input_2.txt";
    println!("In file {file_path}");

    let binding = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let contents = binding.lines().map(|x| {
        x.split_whitespace()
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let good_levels = contents.map(|level| (0..=level.len()).rev().any(|i| is_safe(&level, i)));

    let ans = good_levels.filter(|x| *x).count();

    println!("{:?}", ans);
}
