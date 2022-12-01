use std::fs;
use std::collections::VecDeque;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input file");
    let input: Vec<&str> = input.lines().collect();
    let input: Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut increase_count = 0;
    let mut last_sum = input[0] + input[1] + input[2];
    let mut last_depths: VecDeque<i32> = VecDeque::new();
    for depth in input.iter() {
        last_depths.push_front(*depth);

        if last_depths.len() < 3 {
            continue;
        }

        let sum: i32 = last_depths.iter().sum();
        if sum > last_sum {
            increase_count += 1;
        }
        last_sum = sum;

        last_depths.pop_back();
    }

    println!("Increase count: {}", increase_count);
}
