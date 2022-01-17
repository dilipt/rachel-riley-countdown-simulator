use std::env;
use rand::Rng;
use std::convert::TryFrom;

#[derive(Debug)]
struct Inputs {
    bigs: Vec<u32>,
    smalls: Vec<u32>,
    target: u32,
}

fn main() {
    if env::args().len() != 3 {
        eprintln!("usage: cargo run <bigs> <smalls>");
        std::process::exit(1);
    }
    let args: Vec<u32> = env::args().skip(1).map(|num| num.parse::<u32>().unwrap()).collect();
    let numbers = match args.as_slice() {
        &[bigs, smalls, ..] => (bigs, smalls),
        _ => unreachable!(),
    };
    println!("Got arguments: {:?}", numbers);

    let inputs = generate_inputs(numbers);
    println!("Got inputs: {:?}", inputs);
}

fn generate_inputs(input: (u32, u32)) -> Inputs {
    let bigs_choices = vec![25, 50, 75, 100];
    let smalls_choices = (1..=10).flat_map(|n| vec![n, n]).collect();
    let mut rng = rand::thread_rng();

    return Inputs {
        bigs: generate_input(input.0, bigs_choices, rng),
        smalls: generate_input(input.1, smalls_choices, rng),
        target: rng.gen_range(100, 1000),
    };
}

fn generate_input(n: u32, choices: Vec<u32>, mut rng: rand::prelude::ThreadRng) -> Vec<u32> {
    if n == u32::try_from(choices.len()).unwrap() {
        return choices;
    }
    
    let mut rolls = Vec::<usize>::new();
    let len = choices.len();
    while rolls.len() < usize::try_from(n).unwrap() {
        let roll = rng.gen_range(0, len);
        if !rolls.contains(&roll) {
            rolls.push(roll);
        }
    }
    rolls.iter().map(|roll| choices[*roll]).collect()
}
