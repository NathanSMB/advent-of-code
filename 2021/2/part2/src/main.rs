use std::fs;

struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input file");
    let input: Vec<&str> = input.lines().collect();
    let mut sub = Submarine { x: 0, y: 0, aim: 0 };
    
    for line in input {
        let command: Vec<&str> = line.split(" ").collect();
        let direction = command[0];
        let value = command[1].parse::<i32>().unwrap();
        
        match direction {
            "forward" => {
                sub.x += value;
                sub.y += sub.aim * value;
            },
            "down" => sub.aim += value,
            "up" => sub.aim -= value,
            _ => println!("Invalid direction"),
        }
    }
    
    let result = sub.x * sub.y;
    println!("Result: {}", result);
}
