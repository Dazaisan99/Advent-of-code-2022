use std::io::stdin;

fn main() {
    let lines = input();
    let mut count: usize = 0;

    let mut n_list = Vec::new();

    for line in lines {
        match line.trim() {
            "" => {
                n_list.push(count);
                count = 0;
                continue
            },
            s => {
                count += s.to_string().parse::<usize>().unwrap();
                println!("{}", s);
            }
        }
    }

    n_list.sort();

    println!("{:#?}", n_list[n_list.len()-3 ..].iter().sum::<usize>());
}

// get the input from the input file
fn input() -> Vec<String> {
    let lines = stdin().lines();
    lines.into_iter().map(|s| s.unwrap()).collect::<Vec<String>>()
}
