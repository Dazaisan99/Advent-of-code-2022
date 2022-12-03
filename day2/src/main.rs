use std::io::stdin;

#[derive(Debug)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

enum Win {
    Win,
    Tie,
    Loss,
}

struct Turn {
    opponent: Moves,
    player_need: Win,
}

fn main() {
    let turns = input();
    let mut count: usize = 0;

    for turn in turns {
        let player_move = player_win(turn.opponent, &turn.player_need);
        count += match player_move {
            Moves::Rock => 1,
            Moves::Paper => 2,
            Moves::Scissors => 3,
        };
        count += match turn.player_need {
            Win::Win => 6,
            Win::Tie => 3,
            Win::Loss => 0,
        };
    }

    println!("{}", count);
}

fn input() -> Vec<Turn> {
    let lines = stdin().lines();
    let mut moves_vec: Vec<Turn> = Vec::new();

    for line in lines {
        let s = line.unwrap();
        let moves = s.split_whitespace().into_iter().collect::<Vec<&str>>();
        
        let turn = Turn {
            opponent: match moves[0] {
                "A" => Moves::Rock,
                "B" => Moves::Paper,
                "C" => Moves::Scissors,
                _ => Moves::Scissors,
            },
            player_need: match moves[1] {
                "X" => Win::Loss,
                "Y" => Win::Tie,
                "Z" => Win::Win,
                _ => Win::Win,
        }};

        moves_vec.push(turn);
    }
    moves_vec
}

fn player_win(opponent: Moves, need: &Win) -> Moves {
    match need {
        Win::Win => {
            match opponent {
                Moves::Rock => Moves::Paper,
                Moves::Paper => Moves::Scissors,
                Moves::Scissors => Moves::Rock,
            }
        },
        Win::Tie => opponent,
        Win::Loss => {
            match opponent {
                Moves::Rock => Moves::Scissors,
                Moves::Paper => Moves::Rock,
                Moves::Scissors => Moves::Paper,
            }
        },
    }
}
