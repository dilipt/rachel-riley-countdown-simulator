use std::env;
use rand::Rng;

#[derive(Debug)]
struct Inputs {
    bigs: Vec<u32>,
    smalls: Vec<u32>,
    target: u32,
}

fn main() {
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
    let bigs_choices = [25, 50, 75, 100];
    let mut rng = rand::thread_rng();

    return Inputs {
        bigs: (1..=input.0).map(|_| bigs_choices[rng.gen_range(0, 4)]).collect(),
        smalls: (1..=input.1).map(|_| rng.gen_range(1, 11)).collect(),
        target: rng.gen_range(100, 1000),
    };
}
