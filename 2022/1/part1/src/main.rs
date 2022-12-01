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

    // Find the elf with the most food
    let mut max_calories = 0;
    for elf in elves {
        let calories = elf.food.iter().sum::<i32>();
        if calories > max_calories {
            max_calories = calories;
        }
    }

    println!("The elf with the most food is holding {} calories", max_calories);
}
