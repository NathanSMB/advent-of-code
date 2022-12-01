use std::fs;

// The food vector is a vector of calorie counts for each food item
struct Elf {
    food: Vec<i32>,
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading input file");
    let input = input.split("\n\r\n").collect::<Vec<&str>>();

    // Move the input to the elf data structure
    let mut elves: Vec<Elf> = Vec::new();
    for elf_items in input {
        let elf_items: Vec<&str> = elf_items.lines().collect();
        let elf_items: Vec<i32> = elf_items.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        elves.push(Elf { food: elf_items });
    }

    // Sort the elves by the sum of their food
    elves.sort_by(|a, b| b.food.iter().sum::<i32>().cmp(&a.food.iter().sum::<i32>()));
   
    // Add the top 3 elves' calorie totals together
    let top_three = elves[0].food.iter().sum::<i32>() + elves[1].food.iter().sum::<i32>() + elves[2].food.iter().sum::<i32>();
    
    println!("The three elves holding the most food are holding {} calories total.", top_three);
}
