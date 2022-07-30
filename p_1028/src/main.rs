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

        let _lowest: u64 = if cards_ricardo > cards_vicent {
            cards_vicent
        } else {
            cards_ricardo
        };

        let mut biggest_hand = 0;
        for divisor in 1..(_lowest + 1) {
            let _ricardo = cards_ricardo as f64 / divisor as f64;
            let _vicent = cards_vicent as f64 / divisor as f64;

            if _ricardo != (_ricardo as u32) as f64 || _vicent != (_vicent as u32) as f64 {
                continue;
            } else {
                if divisor > biggest_hand {
                    biggest_hand = divisor;
                }
            };
        }

        println!("{}", biggest_hand);
    }
}
