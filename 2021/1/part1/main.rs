use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input file");
    let input: Vec<&str> = input.lines().collect();
    let input: Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut increase_count = 0;
    let mut last_depth = input[0];
    for depth in input.iter() {
        if depth > &last_depth {
            increase_count += 1;
        }
        last_depth = *depth;
    }

    println!("Increase count: {}", increase_count);
}
