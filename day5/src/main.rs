use std::fs::read_to_string;

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn push(&mut self, item: char) {
        self.crates.push(item);
    }
}

struct Stacks {
    stacks: Vec<Stack>,
}

impl std::str::FromStr for Stacks {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.lines().into_iter().rev();

        let stack_count = input
            .next().unwrap()
            .trim()
            .split("   ")
            .collect::<Vec<&str>>()
            .len();
        
        let mut stacks: Self = Self {
            stacks: vec![Stack {
                crates: Vec::new(),
            }; stack_count],
        };

        for line in input {
            let items = line.chars().skip(1).step_by(4);

            for (index, item) in items.enumerate() {
                if ('A'..='Z').contains(&item) {
                    stacks.stacks[index].push(item as char);
                }
            }
        }

        Ok(stacks)
    }
}

impl Stacks {
    fn pop_back(&mut self, stack: usize) -> char {
        self.stacks[stack].crates.pop().unwrap()
    }

    fn push(&mut self, name_crate: char, to: usize) {
        self.stacks[to].crates.push(name_crate);
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let chara = self.pop_back(from);
        self.push(chara, to)
    }

    fn last_of(&self, of: usize) -> char {
        self.stacks[of].crates[&self.stacks[of].crates.len()-1]
    }

    fn top_crates(&mut self) -> String {
        let mut chars: Vec<char> = Vec::new();
        for (i_stack, _) in self.stacks.iter().enumerate() {
            let s = self.last_of(i_stack);
            chars.push(s);
        }
        String::from_iter(chars.iter())
    }
}

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (number, from, to) = input.strip_prefix("move ")
            .and_then(|input| {
                let (quantity, input) = input.split_once(' ').unwrap();
                let input = input.strip_prefix("from ")?;
                let (from, input) = input.split_once(' ')?;
                let to = input.strip_prefix("to ").unwrap();
                Some((quantity, from, to))
            }).unwrap();
        
        println!("{} {} {}", number, from, to);

        Ok(Self { quantity: number.parse::<usize>().unwrap(), from: from.parse::<usize>().unwrap(), to: to.parse::<usize>().unwrap() })
    }
}

impl Instruction {
    fn execute(&self, stacks: &mut Stacks) {
        let len_stack = stacks.stacks[self.from-1].crates.len();
        let to_move = stacks.stacks[self.from-1].crates[len_stack-self.quantity..].iter().map(|i| *i).collect::<Vec<char>>();

        for i in to_move.into_iter() {
            let _ = stacks.pop_back(self.from-1);
            stacks.push(i.clone(), self.to-1)
        }
    }
}

fn main() {
    let (mut stacks, instructions) = input(); 

    for instruction in instructions {
        instruction.execute(&mut stacks);
    }

    println!("{}", stacks.top_crates());
}

fn input() -> (Stacks, Vec<Instruction>) {
    let input = read_to_string("input").unwrap();
    let (stacks_str, moves_str) = input.split_once("\n\n").unwrap();

    let stacks = stacks_str.parse::<Stacks>().unwrap();
    let moves = moves_str.lines().map(|line| line.parse::<Instruction>().unwrap()).collect::<Vec<Instruction>>();

    (stacks, moves)
}
