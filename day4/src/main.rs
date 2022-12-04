use std::io::stdin;

fn main() {
    let lines = input();
    let mut count: usize = 0;
    for line in lines {
        count += determine_if_contained(&line[0], &line[1]);
    }
    println!("{}", count);
}

// returns a vec that contains 
// each line<a vector containing the two intervals<Each interval<the numbers of the interval>>>
fn input() -> Vec<Vec<Vec<usize>>> {
    let lines = stdin().lines().into_iter();
    let lines_vec = lines.map(|s|
        s.unwrap().split(",").map(|i|
            i.split("-").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        ).collect::<Vec<Vec<usize>>>()
    ).collect::<Vec<Vec<Vec<usize>>>>();

    lines_vec
}

//return 1 if yes
fn determine_if_contained(first_interval: &Vec<usize>, second_interval: &Vec<usize>) -> usize { 
    println!("{:?}, {:?}", first_interval, second_interval);
    let range1 = first_interval[0]..=first_interval[1];
    let range2 = second_interval[0]..=second_interval[1];

    if range1.contains(&second_interval[0]) | range1.contains(&second_interval[1]) {
        return 1;
    } else if range2.contains(&first_interval[0]) | range2.contains(&first_interval[1]) {
    return 1;
    }
    0
}
