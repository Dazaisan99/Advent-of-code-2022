use std::io::stdin;
use std::collections::HashSet;

fn main() {
    println!("{}", get_start_of_message_marker(input(), 14));
}

//gloriously *contributed* code from michael-long88 at github because I was lazy and two days late
fn get_start_of_message_marker(input: String, offset: usize) -> u32 {
    let mut sequence_length: usize = 0;
    let packet_markers = input
        .chars()
        .collect::<Vec<char>>();

    for (index, window) in packet_markers.windows(offset).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == offset {
            sequence_length = index + offset;
            break;
        }
    }

    sequence_length as u32
}

fn input() -> String {
    stdin().lines().next().unwrap().unwrap()
}
