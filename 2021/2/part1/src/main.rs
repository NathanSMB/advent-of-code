use std::fs;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input file");
    let input: Vec<&str> = input.lines().collect();
    let mut current_position = Point { x: 0, y: 0 };
    
    for line in input {
        let command: Vec<&str> = line.split(" ").collect();
        let direction = command[0];
        let value = command[1].parse::<i32>().unwrap();
        
        match direction {
            "forward" => current_position.x += value,
            "down" => current_position.y += value,
            "up" => current_position.y -= value,
            _ => println!("Invalid direction"),
        }
    }
    
    let result = current_position.x * current_position.y;
    println!("Result: {}", result);
}
