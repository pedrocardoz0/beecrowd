use std::io;

fn main() {
    let mut number_of_interactions = String::new();

    io::stdin().read_line(&mut number_of_interactions).unwrap();

    let _number_of_interactions: u32 = number_of_interactions.trim().parse().unwrap();

    for _ in 0.._number_of_interactions {
        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();

        let result: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let cards_ricardo: u64 = result[0];
        let cards_vicent: u64 = result[1];
    }
}
