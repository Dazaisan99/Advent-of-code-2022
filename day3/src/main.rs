use std::io::stdin;
use std::collections::HashSet;

fn main() {
    let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut lines_iter = lines.iter();
    let mut count: usize = 0;

    for _ in 0..(lines_iter.len()/3) as usize {
        let first = lines_iter.next().unwrap().chars().collect();
        let sec = lines_iter.next().unwrap().chars().collect();
        let third = lines_iter.next().unwrap().chars().collect();
        count += get_repeating_char_priority(&first, &sec, &third);
    }
    println!("{}", count);
}

fn priority(chara: char) -> usize {
    match chara.is_lowercase() {
        true => (chara as u8 - 96) as usize,
        false => (chara as u8 - 38) as usize,
    }
}

fn get_repeating_char_priority(first_part: &HashSet<char>, second_part: &HashSet<char>, third_part: &HashSet<char>) -> usize {
    println!("{:?}, {:?}", first_part, second_part);
    priority(*first_part.intersection(second_part).map(|i| *i).collect::<HashSet<char>>().intersection(third_part).next().unwrap())
}
