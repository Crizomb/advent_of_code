use std::fs;

pub fn solve_day_2_1() {
    let file_path = "/home/clement/RustroverProjects/advent_of_code/src/inputs/input_2.txt";
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let first_diff_sign = contents.iter().map(|vec| vec[1] - vec[0] > 0);

    let is_safe_list = contents
        .iter()
        .map(|vec| vec.windows(2))
        .zip(first_diff_sign)
        .map(|(mut vec_win, should_increase)| {
            vec_win.all(|win| {
                (win[1] - win[0]).abs() <= 3
                    && (win[1] - win[0] > 0) == should_increase
                    && win[1] != win[0]
            })
        });

    let ans = is_safe_list.filter(|x| *x).count();

    println!("{:?}", ans);
}
