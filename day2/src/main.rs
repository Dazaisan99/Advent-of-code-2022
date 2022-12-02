use std::io::stdin;

enum Moves {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    input();
}

fn input() -> Vec<Vec<Moves>> {
    let mut lines = stdin().lines();

    //for each turn
    for line in lines {
        //convert to moves here
        println!("{:#?}", line.unwrap().split_whitespace().collect::<Vec<&str>>());
    }
    // placeholder
    vec![vec![Moves::Paper]]
}
