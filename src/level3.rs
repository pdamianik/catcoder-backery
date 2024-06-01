use enum_map::{Enum, EnumMap};
use log::{debug, info};
use owo_colors::OwoColorize;

const EXERCISES: [&str; 5] = [
    "F 1 209 F 2 254 F 3 895 F 4 439 B 1 104 B 2 127 B 3 74 B 3 447 B 4 127 B 4 219 B 5 448 B 6 220",
    "F 1 367 F 2 38 F 3 602 F 4 624 B 1 183 B 2 19 B 3 184 B 3 301 B 4 19 B 4 312 B 5 64 B 6 312",
    "F 1 995 F 2 884 F 3 749 F 4 866 B 1 497 B 2 442 B 3 319 B 3 374 B 4 239 B 4 433 B 5 375 B 6 177",
    "F 1 123395 F 2 488034 F 3 78861 F 4 200882 F 5 102517 F 6 49658 F 7 201804 F 8 247860 F 9 356333 F 10 163982 F 11 351283 F 12 305592 F 13 443860 F 14 111094 F 15 216152 F 17 220897 F 16 184823 F 19 438974 F 18 464208 B 1 61697 B 2 9577 B 2 244017 B 3 30849 B 3 39430 B 4 244017 B 4 19716 B 4 100441 B 5 19715 B 5 51258 B 6 100441 B 6 25630 B 6 24829 B 7 25629 B 7 100902 B 8 24829 B 8 123930 B 9 100902 B 9 61965 B 9 178166 B 10 61965 B 10 89084 B 10 81991 B 11 89083 B 11 175641 B 12 81991 B 12 152796 B 13 175642 B 13 76398 B 13 221930 B 14 76398 B 14 110965 B 14 55547 B 15 110965 B 15 27774 B 15 108076 B 17 73709 B 17 46206 B 17 110448 B 16 27773 B 16 92411 B 19 110449 B 19 116052 B 19 219487 B 18 46206 B 18 232104 B 21 219487 B 20 116052",
    "F 1 114137 F 2 351326 F 3 368940 F 4 149918 F 5 392828 F 6 170391 F 7 322537 F 8 375036 F 9 326464 F 10 466616 F 11 435944 F 12 36944 F 13 328942 F 14 291904 F 15 309906 F 17 359944 F 16 173875 F 19 224204 F 18 258166 B 1 57068 B 2 175663 B 3 57069 B 3 87832 B 3 184470 B 4 87831 B 4 74959 B 5 184470 B 5 31463 B 5 196414 B 6 37479 B 6 85195 B 7 196414 B 7 161268 B 8 85196 B 8 80635 B 8 187518 B 9 80634 B 9 163232 B 10 187518 B 10 78201 B 10 233308 B 11 81616 B 11 217972 B 12 233308 B 12 108986 B 12 18472 B 13 108986 B 13 9236 B 13 164471 B 14 9236 B 14 82236 B 14 145952 B 15 82235 B 15 154953 B 17 154953 B 17 34084 B 17 179972 B 16 145952 B 16 86937 B 19 179972 B 19 64542 B 19 112102 B 18 43469 B 18 129083 B 21 56051 B 20 64541 B 20 6255",
];

const MAX_PAYMENTS: usize = 4;

#[derive(Debug, Eq, PartialEq, Enum)]
enum TransferType {
    Sale,
    Payment,
}

#[derive(Debug, Eq, PartialEq)]
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
    let max_sale_day = transfers[TransferType::Sale].iter()
        .map(|transfer| transfer.day)
        .max().expect("No sales were parsed");
    let max_payment_day = transfers[TransferType::Payment].iter()
        .map(|transfer| transfer.day)
        .max().expect("No payments were parsed");

    let mut sales: Vec<usize> = vec![0; max_sale_day];
    for sale in &transfers[TransferType::Sale] {
        sales[sale.day - 1] = sale.amount;
    }

    let mut payments: Vec<usize> = vec![0; max_payment_day * MAX_PAYMENTS];
    for payment in &transfers[TransferType::Payment] {
        *payments[(payment.day - 1) * MAX_PAYMENTS..payment.day * MAX_PAYMENTS].iter_mut()
            .find(|&&mut payment| payment == 0)
            .expect(&format!("More than {} payments for day {}", MAX_PAYMENTS, payment.day))
            = payment.amount;
    }

    let mut available_removals = Vec::new();

    for day in 1..=max_sale_day {
        let count = payments[(day - 1) * MAX_PAYMENTS..day * MAX_PAYMENTS].iter()
            .enumerate()
            .find(|(_, &val)| val == 0)
            .map(|(index, _)| index)
            .unwrap_or(MAX_PAYMENTS);
        let day_sale = sales[day - 1];
        let day_payment: usize = payments[(day - 1) * MAX_PAYMENTS..day * MAX_PAYMENTS].iter().sum();
        let remaining = day_sale - day_payment;
        let remaining_count = MAX_PAYMENTS - count;

        debug!("{}: {}, {}: {}, {}: {}, {}: {}, {}: {}",
            "day".bold(), day,
            "sales".bold(), sales[day - 1],
            "payment".bold(), day_payment,
            "remaining".bold(), remaining,
            "remaining_count", remaining_count);

        if remaining_count == 0 || remaining == 0 {
            continue;
        }

        let (removes, adds) = find_reordering(day, &payments, sales[day - 1], count, available_removals.len());

        for &remove in &removes {
            let mut uses_remaining = true;
            if let Some((target_index, target_count)) = available_removals.last_mut() {
                payments[*target_index] = payments[remove];
                payments[remove] = 0;

                *target_index += 1;
                *target_count -= 1;
                uses_remaining = *target_count > 0;
            }
            if !uses_remaining {
                available_removals.pop();
            }
        }

        let mut insert = (day - 1) * MAX_PAYMENTS;
        let mut count = 0;
        let mut sum = 0;
        for &add in &adds {
            if add == insert {
                insert += 1;
                continue;
            }
            sum += payments[add];
            payments[insert] = payments[add];
            insert += 1;
            payments[add] = 0;
            count += 1;

            for reshuffle in add + 1..=add + MAX_PAYMENTS - add % MAX_PAYMENTS {
                if payments[reshuffle] == 0 {
                    payments[reshuffle - 1] = 0;
                    break;
                }

                payments[reshuffle - 1] = payments[reshuffle];
            }
        }

        if count < MAX_PAYMENTS && sum != day_sale {
            available_removals.push((day - 1 + count, MAX_PAYMENTS - count));
        }

        debug!("{}: {:?}, {}: {:?}", "removes".bold(), &removes, "adds".bold(), &adds);
    }

    let days = sales.iter()
        .enumerate()
        .filter_map(|(day, &amount)| {
            if amount != payments[day * 4..(day + 1) * 4].iter().sum() {
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

fn find_reordering(day: usize, payments: &[usize], sale: usize, existing_payments: usize, available_removals: usize) -> (Vec<usize>, Vec<usize>) {
    let search_start = (day - 1) * MAX_PAYMENTS;
    let potential_payments = payments[search_start..].iter().enumerate()
        .filter(|(_, &val)| 0 < val && val <= sale)
        .map(|(index, &val)| (index, val))
        .collect::<Vec<_>>();

    let count = potential_payments.len();
    let mut selected = (vec![], vec![]);
    let mut necessary_removals = vec![true; existing_payments];
    let mut min_adds = usize::MAX;

    search_reordering_element(
        MAX_PAYMENTS - 1,
        0,
        count,
        sale,
        &potential_payments,
        &mut Vec::with_capacity(MAX_PAYMENTS),
        &mut 0,
        &mut necessary_removals,
        available_removals,
        &mut min_adds,
        &mut selected,

    );

    let (removes, adds) = selected;

    let removes = removes.iter()
        .enumerate()
        .filter(|(_, &val)| val)
        .map(|(index, _)| search_start + index)
        .collect();

    let adds = adds.iter()
        .map(|index| search_start + index)
        .collect();

    return (removes, adds);
}

fn search_reordering_element(
    considered_index: usize,
    start: usize,
    count: usize,
    sale: usize,
    potential_payments: &[(usize, usize)],
    current_payments: &mut Vec<usize>,
    sum: &mut usize,
    necessary_removals: &mut Vec<bool>,
    available_removals: usize,
    min_adds: &mut usize,
    selected: &mut (Vec<bool>, Vec<usize>)
) {
    for considered_payment in start..count {
        let (index, amount) = potential_payments[considered_payment];
        *sum += amount;
        current_payments.push(index);
        if index < MAX_PAYMENTS {
            necessary_removals[index] = false;
        }
        if *sum == sale {
            let removal_count = necessary_removals.iter()
                .filter(|&&val| val)
                .count();
            if removal_count <= available_removals && current_payments.len() < *min_adds {
                *min_adds = current_payments.len();
                *selected = (necessary_removals.clone(), current_payments.clone());
            }
        } else if *sum < sale && considered_index > 0 {
            search_reordering_element(
                considered_index - 1,
                considered_payment + 1,
                count,
                sale,
                potential_payments,
                current_payments,
                sum,
                necessary_removals,
                available_removals,
                min_adds,
                selected,
            );
        }

        *sum -= amount;
        current_payments.pop();
        if index < MAX_PAYMENTS {
            necessary_removals[index] = true;
        }
    }
}

#[test]
fn example() {
    let input = "F 1 200 F 2 170 B 1 100 B 2 80 B 2 15 B 2 100 B 3 70";
    let output = solve(input);
    assert_eq!(output, "2");
}
