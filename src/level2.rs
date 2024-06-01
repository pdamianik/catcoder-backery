use enum_map::{Enum, EnumMap};
use log::info;
use owo_colors::OwoColorize;

const EXERCISES: [&str; 5] = [
    "F 1 70279 F 2 71697 F 3 72312 F 4 92962 F 5 69037 F 6 64701 F 7 15735 F 8 63881 F 9 85432 F 10 49312 F 11 88476 F 12 57879 F 13 711 F 14 3047 F 15 89578 F 17 90534 F 16 46159 F 19 12319 F 18 80732 F 21 68589 F 20 13695 F 23 64955 F 22 99126 F 25 83418 F 24 47564 F 27 62666 F 26 1415 F 29 56923 F 28 5462 F 31 63549 F 30 23683 F 34 30309 F 35 7806 F 32 16785 F 33 76118 F 38 3961 F 39 84460 F 36 77193 F 37 83240 F 42 89059 F 43 29384 F 40 31573 F 41 15943 F 46 18373 F 47 36839 F 44 70101 F 45 33254 F 51 87891 F 50 58823 F 49 76613 F 48 45113 F 55 77080 F 54 34390 F 53 7002 F 52 87040 F 59 53382 F 58 38086 F 57 50717 F 56 75719 F 63 5327 F 62 93224 F 61 98499 F 60 96760 F 68 88240 F 69 63607 F 70 87648 F 71 77802 F 64 8938 F 65 43678 F 66 95714 F 67 1993 F 76 84351 F 77 24678 F 78 62930 F 79 66493 F 72 18128 F 73 95642 F 74 43932 F 75 82408 F 85 73469 F 84 67909 F 87 17497 F 86 63392 F 81 2032 F 80 78865 F 83 61709 F 82 71615 F 93 38106 F 92 40906 F 95 23661 F 94 56475 F 89 13487 F 88 63915 F 91 48109 F 90 12553 F 98 44019 F 99 64441 F 96 34052 F 97 94889 B 1 70279 B 2 71697 B 3 55700 B 4 92962 B 5 69037 B 6 64701 B 7 15735 B 8 63881 B 9 85432 B 10 49312 B 11 88476 B 12 57879 B 13 711 B 14 3047 B 15 89578 B 17 90534 B 16 46159 B 19 12319 B 18 80732 B 21 68589 B 20 13695 B 23 64955 B 22 99126 B 25 83418 B 24 47564 B 27 62666 B 26 1415 B 29 56923 B 28 5462 B 31 63549 B 30 23683 B 34 30309 B 35 7806 B 32 16785 B 33 76118 B 38 3961 B 39 66173 B 36 77193 B 37 83240 B 42 89059 B 43 29384 B 40 31573 B 41 15943 B 46 18373 B 47 36839 B 44 70101 B 45 33254 B 51 87891 B 50 58823 B 49 76613 B 48 45113 B 55 77080 B 54 34390 B 53 7002 B 52 87040 B 59 53382 B 58 38086 B 57 50717 B 56 75719 B 63 5327 B 62 93224 B 61 98499 B 60 96760 B 68 88240 B 69 63607 B 70 87648 B 71 77802 B 64 8938 B 65 43678 B 66 95714 B 67 1993 B 76 84351 B 77 24678 B 78 62930 B 79 66493 B 72 18128 B 73 95642 B 74 43932 B 75 82408 B 85 73469 B 84 67909 B 87 17497 B 86 63392 B 81 2032 B 80 78865 B 83 61709 B 82 71615 B 93 38106 B 92 40906 B 95 23661 B 94 56475 B 89 13487 B 88 63915 B 91 48109 B 90 12553 B 98 44019 B 99 64441 B 96 34052 B 97 94889",
    "F 1 30438 F 2 54766 F 3 73957 F 4 35783 F 5 56929 F 6 7532 F 7 43281 F 8 25080 F 9 98517 F 10 45512 F 11 22824 F 12 35763 F 13 4377 F 14 43445 F 15 84312 F 17 10563 F 16 16842 F 19 83703 F 18 52792 F 21 26744 F 20 1206 F 23 6378 F 22 89565 F 25 23302 F 24 87348 F 27 78549 F 26 48135 F 29 5020 F 28 35577 F 31 5983 F 30 53066 F 34 74059 F 35 66073 F 32 56369 F 33 92220 F 38 24899 F 39 32138 F 36 71019 F 37 11443 F 42 34189 F 43 19201 F 40 22857 F 41 85791 F 46 91854 F 47 43347 F 44 73697 F 45 89178 F 51 7883 F 50 93888 F 49 60497 F 48 75877 F 55 77100 F 54 52828 F 53 7798 F 52 30996 F 59 35728 F 58 28700 F 57 18208 F 56 52594 F 63 54634 F 62 76425 F 61 21320 F 60 20225 F 68 70357 F 69 14332 F 70 53093 F 71 12484 F 64 51108 F 65 30512 F 66 6634 F 67 48008 F 76 37154 F 77 37682 F 78 6700 F 79 84348 F 72 80669 F 73 19401 F 74 18502 F 75 44334 F 85 48408 F 84 80094 F 87 23151 F 86 58758 F 81 16086 F 80 23121 F 83 37657 F 82 47717 F 93 33897 F 92 45612 F 95 6440 F 94 16848 F 89 53515 F 88 31290 F 91 47315 F 90 23057 F 98 64046 F 99 20222 F 96 70490 F 97 91342 B 1 30438 B 2 54766 B 3 73957 B 4 35783 B 5 56929 B 6 7532 B 7 43281 B 8 25080 B 9 98517 B 10 45512 B 11 22824 B 12 35763 B 13 4377 B 14 43445 B 15 84312 B 17 10563 B 16 16842 B 19 83703 B 18 52792 B 21 26744 B 20 1206 B 23 6378 B 22 89565 B 25 18919 B 24 87348 B 27 78549 B 26 48135 B 29 5020 B 28 35577 B 31 5983 B 30 53066 B 34 74059 B 35 20882 B 32 56369 B 33 92220 B 38 24899 B 39 32138 B 36 71019 B 37 11443 B 42 34189 B 43 19201 B 40 22857 B 41 85791 B 46 24804 B 47 43347 B 44 73697 B 45 89178 B 51 7883 B 50 93888 B 49 60497 B 48 6189 B 55 77100 B 54 52828 B 53 7798 B 52 30996 B 59 35728 B 58 28700 B 57 18208 B 56 52594 B 63 54634 B 62 76425 B 61 21320 B 60 20225 B 68 70357 B 69 14332 B 70 53093 B 71 12484 B 64 51108 B 65 30512 B 66 6634 B 67 48008 B 76 37154 B 77 37682 B 78 6700 B 79 84348 B 72 80669 B 73 19401 B 74 18502 B 75 44334 B 85 48408 B 84 80094 B 87 23151 B 86 58758 B 81 16086 B 80 23121 B 83 37657 B 82 47717 B 93 33897 B 92 45612 B 95 6440 B 94 16848 B 89 53515 B 88 31290 B 91 47315 B 90 23057 B 98 64046 B 99 20222 B 96 70490 B 97 91342",
    "F 1 79318 F 2 81994 F 3 89081 F 4 86744 F 5 41304 F 6 70559 F 7 79598 F 8 8995 F 9 85308 F 10 46431 F 11 72537 F 12 5184 F 13 74963 F 14 3699 F 15 81797 F 17 31063 F 16 18480 F 19 90128 F 18 30833 F 21 1804 F 20 7580 F 23 54639 F 22 1592 F 25 20593 F 24 42817 F 27 31291 F 26 48114 F 29 68582 F 28 21588 F 31 94186 F 30 9248 F 34 94746 F 35 39577 F 32 86794 F 33 81807 F 38 47979 F 39 48238 F 36 12053 F 37 76198 F 42 36355 F 43 25841 F 40 31926 F 41 82058 F 46 6859 F 47 15310 F 44 99184 F 45 57977 F 51 19849 F 50 14289 F 49 24556 F 48 76435 F 55 50832 F 54 80749 F 53 17960 F 52 21301 F 59 71610 F 58 12255 F 57 47693 F 56 93105 F 63 92537 F 62 84391 F 61 57065 F 60 15425 F 68 84479 F 69 59317 F 70 29167 F 71 1431 F 64 97393 F 65 17568 F 66 27294 F 67 84880 F 76 42207 F 77 30990 F 78 49902 F 79 22897 F 72 21374 F 73 6744 F 74 44113 F 75 55822 F 85 45040 F 84 95114 F 87 21104 F 86 20379 F 81 46829 F 80 41589 F 83 79160 F 82 28312 F 93 37202 F 92 54430 F 95 92031 F 94 345 F 89 40386 F 88 48708 F 91 48151 F 90 44234 F 98 39766 F 99 75724 F 96 46495 F 97 83095 B 1 79318 B 2 81994 B 3 89081 B 4 86744 B 5 41304 B 6 70559 B 7 79598 B 8 5115 B 9 85308 B 10 46431 B 11 72537 B 12 5184 B 13 74963 B 14 3699 B 15 81797 B 17 31063 B 16 18480 B 19 90128 B 18 30833 B 21 1804 B 20 7580 B 23 54639 B 22 1592 B 25 20593 B 24 42817 B 27 31291 B 26 48114 B 29 68582 B 28 21588 B 31 94186 B 30 9248 B 34 94746 B 35 39577 B 32 86794 B 33 81807 B 38 47979 B 39 48238 B 36 12053 B 37 76198 B 42 36355 B 43 25841 B 40 31926 B 41 82058 B 46 6859 B 47 15310 B 44 99184 B 45 57977 B 51 19849 B 50 14289 B 49 24556 B 48 76435 B 55 50832 B 54 80749 B 53 17960 B 52 21301 B 59 71610 B 58 11502 B 57 47693 B 56 93105 B 63 92537 B 62 84391 B 61 57065 B 60 15425 B 68 84479 B 69 59317 B 70 29167 B 71 1431 B 64 97393 B 65 17568 B 66 27294 B 67 84880 B 76 42207 B 77 7422 B 78 49902 B 79 22897 B 72 21374 B 73 6744 B 74 44113 B 75 55822 B 85 38868 B 84 95114 B 87 21104 B 86 20379 B 81 46829 B 80 41589 B 83 79160 B 82 28312 B 93 37202 B 92 54430 B 95 92031 B 94 345 B 89 40386 B 88 48708 B 91 48151 B 90 44234 B 98 39766 B 99 75724 B 96 46495 B 97 83095",
    "F 1 77935 F 2 18343 F 3 1456 F 4 99896 F 5 63031 F 6 48372 F 7 69467 F 8 9866 F 9 63249 F 10 69694 F 11 58309 F 12 93812 F 13 26678 F 14 34718 F 15 88121 F 17 24434 F 16 21412 F 19 69427 F 18 57243 F 21 55433 F 20 80215 F 23 95946 F 22 70997 F 25 93326 F 24 71164 F 27 69576 F 26 8013 F 29 60798 F 28 2648 F 31 93583 F 30 68634 F 34 47085 F 35 72034 F 32 37963 F 33 36795 F 38 99436 F 39 53279 F 36 67751 F 37 21594 F 42 29995 F 43 91558 F 40 3409 F 41 29655 F 46 32237 F 47 5765 F 44 64458 F 45 61973 F 51 17225 F 50 46717 F 49 71856 F 48 10140 F 55 30425 F 54 93022 F 53 23152 F 52 79944 F 59 88212 F 58 94839 F 57 38239 F 56 72106 F 63 33027 F 62 48898 F 61 16854 F 60 47344 F 68 84151 F 69 7088 F 70 26485 F 71 67654 F 64 74838 F 65 20987 F 66 9659 F 67 30838 F 76 37652 F 77 88419 F 78 15255 F 79 44050 F 72 99698 F 73 67348 F 74 84799 F 75 49811 F 85 51357 F 84 36670 F 87 7144 F 86 48536 F 81 14028 F 80 46378 F 83 46307 F 82 37658 F 93 94563 F 92 56189 F 95 36191 F 94 6184 F 89 1306 F 88 86620 F 91 8302 F 90 57580 F 98 24974 F 99 21889 F 96 36659 F 97 37662 B 1 77935 B 2 18343 B 3 1456 B 4 99896 B 5 63031 B 6 48372 B 7 69467 B 8 9866 B 9 63249 B 10 69694 B 11 58309 B 12 93812 B 13 26678 B 14 34718 B 15 88121 B 17 24434 B 16 21412 B 19 69427 B 18 57243 B 21 55433 B 20 80215 B 23 95946 B 22 70997 B 25 93326 B 24 71164 B 27 69576 B 26 8013 B 29 60798 B 28 2648 B 31 93583 B 30 68634 B 34 47085 B 35 72034 B 32 37963 B 33 36795 B 38 99436 B 39 53279 B 36 67751 B 37 21594 B 42 29995 B 43 91558 B 40 3409 B 41 29655 B 46 32237 B 47 5765 B 44 64458 B 45 61973 B 51 17225 B 50 46717 B 49 71856 B 48 10140 B 55 30425 B 54 93022 B 53 23152 B 52 79944 B 59 88212 B 58 94839 B 57 38239 B 56 72106 B 63 33027 B 62 48898 B 61 16854 B 60 47344 B 68 84151 B 69 7088 B 70 26485 B 71 67654 B 64 74838 B 65 20987 B 66 9659 B 67 30838 B 76 37652 B 77 88419 B 78 15255 B 79 44050 B 72 99698 B 73 67348 B 74 84799 B 75 49811 B 85 51357 B 84 36670 B 87 7144 B 86 48536 B 81 14028 B 80 46378 B 83 46307 B 82 37658 B 93 94563 B 92 56189 B 95 36191 B 94 6184 B 89 1306 B 88 86620 B 91 8302 B 90 57580 B 98 24974 B 99 21889 B 96 36659 B 97 37662",
    "F 1 68671 F 2 92572 F 3 44609 F 4 70727 F 5 27337 F 6 31910 F 7 20965 F 8 56006 F 9 18597 F 10 80124 F 11 22278 F 12 43122 F 13 88636 F 14 64939 F 15 45179 F 17 99346 F 16 73681 F 19 23871 F 18 61686 F 21 16956 F 20 46044 F 23 29468 F 22 74908 F 25 68553 F 24 34295 F 27 20730 F 26 12133 F 29 5346 F 28 28256 F 31 49854 F 30 33134 F 34 59489 F 35 42469 F 32 95462 F 33 70837 F 38 21360 F 39 2429 F 36 16138 F 37 97300 F 42 79994 F 43 4054 F 40 46428 F 41 32653 F 46 15948 F 47 54595 F 44 55952 F 45 36735 F 51 94327 F 50 42344 F 49 31914 F 48 16413 F 55 10421 F 54 70451 F 53 62486 F 52 86920 F 59 34541 F 58 5747 F 57 28707 F 56 44719 F 63 4696 F 62 51384 F 61 13819 F 60 23653 F 68 80584 F 69 35435 F 70 69238 F 71 92803 F 64 69175 F 65 92656 F 66 69010 F 67 79507 F 76 70802 F 77 33060 F 78 1486 F 79 72340 F 72 18043 F 73 88713 F 74 78076 F 75 10942 F 85 74497 F 84 61039 F 87 86333 F 86 4357 F 81 3322 F 80 82457 F 83 87244 F 82 39637 F 93 34998 F 92 2270 F 95 43010 F 94 29931 F 89 12651 F 88 15473 F 91 75048 F 90 95239 F 98 79804 F 99 36648 F 96 93591 F 97 68641 B 1 68671 B 2 92572 B 3 44609 B 4 70727 B 5 24388 B 6 31910 B 7 20965 B 8 56006 B 9 18597 B 10 80124 B 11 22278 B 12 43122 B 13 88636 B 14 64939 B 15 45179 B 17 99346 B 16 73681 B 19 23871 B 18 61686 B 21 16956 B 20 46044 B 23 29468 B 22 74908 B 25 68553 B 24 34295 B 27 20730 B 26 12133 B 29 5346 B 28 28256 B 31 49854 B 30 33134 B 34 59489 B 35 42469 B 32 88733 B 33 70837 B 38 21360 B 39 2429 B 36 1262 B 37 97300 B 42 79994 B 43 4054 B 40 46428 B 41 32653 B 46 15948 B 47 54595 B 44 55952 B 45 36735 B 51 94327 B 50 42344 B 49 31914 B 48 16413 B 55 10421 B 54 70451 B 53 62486 B 52 86920 B 59 34541 B 58 5747 B 57 28707 B 56 44719 B 63 4696 B 62 51384 B 61 13819 B 60 23653 B 68 80584 B 69 35435 B 70 69238 B 71 92803 B 64 69175 B 65 92656 B 66 69010 B 67 79507 B 76 70802 B 77 33060 B 78 1486 B 79 72340 B 72 18043 B 73 88713 B 74 78076 B 75 10942 B 85 74497 B 84 61039 B 87 86333 B 86 4357 B 81 3322 B 80 82457 B 83 87244 B 82 39637 B 93 34998 B 92 2270 B 95 43010 B 94 29931 B 89 12651 B 88 15473 B 91 75048 B 90 95239 B 98 79804 B 99 36648 B 96 93591 B 97 68641",
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
