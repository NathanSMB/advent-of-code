use std::fs;

#[derive(Clone)]
enum RockPaperScissorSelection {
    Rock,
    Paper,
    Scissor,
}

struct RoundStrategy {
    opponent: RockPaperScissorSelection,
    myself: RockPaperScissorSelection,
}

fn play_and_score(round: RoundStrategy, score: &mut i32) {
    let win_points = 6;
    let tie_points = 3;
    let lose_points = 0;

    match round.myself {
        RockPaperScissorSelection::Rock => {
            *score += 1;
            match round.opponent {
                RockPaperScissorSelection::Rock => *score += tie_points,
                RockPaperScissorSelection::Paper => *score += lose_points,
                RockPaperScissorSelection::Scissor => *score += win_points,
            }
        },
        RockPaperScissorSelection::Paper => {
            *score += 2;
            match round.opponent {
                RockPaperScissorSelection::Rock => *score += win_points,
                RockPaperScissorSelection::Paper => *score += tie_points,
                RockPaperScissorSelection::Scissor => *score += lose_points,
            }
        },
        RockPaperScissorSelection::Scissor => {
            *score += 3;
            match round.opponent {
                RockPaperScissorSelection::Rock => *score += lose_points,
                RockPaperScissorSelection::Paper => *score += win_points,
                RockPaperScissorSelection::Scissor => *score += tie_points,
            }
        },
    }
}


fn main() {
    let input = fs::read_to_string("input").expect("Error reading input file");
    let input: Vec<&str> = input.lines().collect();
    let input: Vec<RoundStrategy> = input
        .iter()
        .map(|line| {
            let mut line = line.split_whitespace();
            let opponent = match line.next().unwrap() {
                "A" => RockPaperScissorSelection::Rock,
                "B" => RockPaperScissorSelection::Paper,
                "C" => RockPaperScissorSelection::Scissor,
                _ => panic!("Invalid input"),
            };
            let myself = match line.next().unwrap() {
                "X" => match opponent {
                    RockPaperScissorSelection::Rock => RockPaperScissorSelection::Scissor,
                    RockPaperScissorSelection::Paper => RockPaperScissorSelection::Rock,
                    RockPaperScissorSelection::Scissor => RockPaperScissorSelection::Paper,
                },
                // When matching for "Y" copy the opponent's selection
                "Y" => opponent.clone(),
                "Z" => match opponent {
                    RockPaperScissorSelection::Rock => RockPaperScissorSelection::Paper,
                    RockPaperScissorSelection::Paper => RockPaperScissorSelection::Scissor,
                    RockPaperScissorSelection::Scissor => RockPaperScissorSelection::Rock,
                },
                _ => panic!("Invalid input"),
            };
            RoundStrategy { opponent, myself }
        })
        .collect();

    let mut my_score = 0;
    for round in input {
        play_and_score(round, &mut my_score);
    }
    println!("{}", my_score);
}
