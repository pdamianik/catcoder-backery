use enum_map::{Enum, EnumMap};
use log::info;
use owo_colors::OwoColorize;

const EXERCISES: [&str; 5] = [
    "F 1 386 F 2 963 F 3 374 F 4 371 F 5 685 F 6 47 F 7 132 F 8 909 F 9 433 B 1 386 B 2 963 B 3 374 B 4 371 B 5 685 B 6 47 B 7 132 B 8 909 B 9 71",
    "F 1 439 F 2 605 F 3 476 F 4 765 F 5 382 F 6 599 F 7 732 F 8 21 F 9 617 B 1 248 B 2 605 B 3 476 B 4 765 B 5 382 B 6 599 B 7 732 B 8 21 B 9 617",
    "F 1 52 F 2 746 F 3 866 F 4 12 F 5 892 F 6 918 F 7 514 F 8 794 F 9 385 B 1 52 B 2 746 B 3 13 B 4 7 B 5 892 B 6 918 B 7 514 B 8 794 B 9 385",
    "F 1 111 F 2 143 F 3 827 F 4 987 F 5 507 F 6 694 F 7 702 F 8 51 F 9 830 B 1 111 B 2 143 B 3 827 B 4 987 B 5 507 B 6 526 B 7 702 B 8 27 B 9 403",
    "F 1 739 F 2 164 F 3 227 F 4 778 F 5 423 F 6 538 F 7 155 F 8 425 F 9 878 B 1 739 B 2 164 B 3 227 B 4 778 B 5 423 B 6 538 B 7 155 B 8 425 B 9 878",
];

#[derive(Debug, Eq, PartialEq, Enum)]
enum TransferType {
    Sale,
    Payment,
}

struct Transfer {
    day: usize,
    amount: usize,
}

pub fn main() {
    for exercise in EXERCISES {
        info!("{}: {:.25} {} {}", "Input".bold(), exercise, exercise.len().to_string().dimmed(), "characters".dimmed());

        let output = solve(exercise);

        info!("{}: {}", "Output".bold(), output);
    }
}

fn solve(exercise: &str) -> String {
    let transfers = parse(exercise);
    let max_day = transfers[TransferType::Sale].iter()
        .map(|transfer| transfer.day)
        .max().unwrap();

    let mut budget = vec![0; max_day];
    for sale in &transfers[TransferType::Sale] {
        budget[sale.day - 1] += sale.amount;
    }

    for payment in &transfers[TransferType::Payment] {
        budget[payment.day - 1] -= payment.amount;
    }

    let days = budget.iter()
        .enumerate()
        .filter_map(|(day, &amount)| {
            if amount > 0 {
                Some((day + 1).to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if days.len() == 0 {
        " ".to_string()
    } else {
        days.join(" ")
    }
}

fn parse(exercise: &str) -> EnumMap<TransferType, Vec<Transfer>> {
    let mut transfers = EnumMap::from_fn(|_| Vec::new());

    let mut elements = exercise.split(' ');

    while let Some(transfer_type) = elements.next() {
        let transfer_type = match transfer_type {
            "F" => TransferType::Sale,
            "B" => TransferType::Payment,
            _ => unreachable!(),
        };

        let day = elements.next().unwrap().parse().unwrap();
        let amount = elements.next().unwrap().parse().unwrap();

        let transfer = Transfer {
            day,
            amount,
        };

        transfers[transfer_type].push(transfer);
    }

    transfers
}

#[test]
fn example() {
    let input = "F 1 200 F 2 170 B 1 200 B 2 100";
    let output = solve(input);
    assert_eq!(output, "2");
}