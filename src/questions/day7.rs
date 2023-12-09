const BASE_LOOKUP_TABLE: [usize; 35] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 9, 11, 0, 0, 0, 0, 0,
    10, 0, 0, 8,
];

const JOKER_LOOKUP_TABLE: [usize; 35] = [
    1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0,
    10, 0, 0, 9,
];

const FIVE_OF_A_KIND: usize = 6;
const FOUR_OF_A_KIND: usize = 5;
const FULL_HOUSE: usize = 4;
const THREE_OF_A_KIND: usize = 3;
const TWO_PAIRS: usize = 2;
const ONE_PAIR: usize = 1;
const HIGH_CARD: usize = 0;

fn insert_into(hands: &mut [Vec<([usize; 5], u32)>; 13], hand: [usize; 5], value: u32) {
    hands[hand[0]].insert(
        hands[hand[0]]
            .binary_search_by(|a| a.0.cmp(&hand))
            .unwrap_or_else(|e| e),
        (hand, value),
    );
}

pub fn part_one(input: &str) -> i128 {
    let mut result: u32 = 0;

    let mut hands: [[Vec<([usize; 5], u32)>; 13]; 7] = Default::default();

    'line: for line in input.lines() {
        let bytes = line.as_bytes();
        let mut count_by_type = [0; 13];
        let mut hand = [0; 5];

        let mut four_of_a_kind = false;
        let mut three_of_a_kind = false;
        let mut two_pairs = false;
        let mut pair = false;

        let value = bytes[6..]
            .iter()
            .map(|&c| c - b'0')
            .fold(0, |acc: u32, f| acc * 10 + f as u32);

        for i in 0..5 {
            hand[i] = BASE_LOOKUP_TABLE[bytes[i] as usize - b'2' as usize];
            count_by_type[hand[i]] += 1;

            match count_by_type[hand[i]] {
                5 => {
                    insert_into(&mut hands[FIVE_OF_A_KIND], hand, value);
                    continue 'line;
                }
                4 => four_of_a_kind = true,
                3 => {
                    if two_pairs {
                        insert_into(&mut hands[FULL_HOUSE], hand, value);
                        continue 'line;
                    }
                    three_of_a_kind = true
                },
                2 => {
                    if three_of_a_kind {
                        insert_into(&mut hands[FULL_HOUSE], hand, value);
                        continue 'line;
                    }
                    if pair {
                        two_pairs = true;
                    }
                    pair = true;
                }
                _ => {}
            }
        }

        if four_of_a_kind {
            insert_into(&mut hands[FOUR_OF_A_KIND], hand, value);
            continue 'line;
        }
        if three_of_a_kind {
            insert_into(&mut hands[THREE_OF_A_KIND], hand, value);
            continue 'line;
        }
        if two_pairs {
            insert_into(&mut hands[TWO_PAIRS], hand, value);
            continue 'line;
        }
        if pair {
            insert_into(&mut hands[ONE_PAIR], hand, value);
            continue 'line;
        }
        insert_into(&mut hands[HIGH_CARD], hand, value);
    }

    let mut position = 1;
    for hands in hands {
        for first_card in hands {
            for hand in first_card {
                result += hand.1 * position;
                position += 1;
            }
        }
    }

    result.into()
}

pub fn part_two(input: &str) -> i128 {
    let mut result: u32 = 0;
    let mut hands: [[Vec<([usize; 5], u32)>; 13]; 7] = Default::default();
    'line: for line in input.lines() {
        let mut count_by_type = [0; 13];
        let bytes = line.as_bytes();
        let mut hand = [0; 5];
        for i in 0..5 {
            hand[i] = JOKER_LOOKUP_TABLE[bytes[i] as usize - b'2' as usize];
            count_by_type[hand[i]] += 1;
        }

        let value = bytes[6..]
            .iter()
            .map(|&c| c - b'0')
            .fold(0, |acc: u32, f| acc * 10 + f as u32);

        let mut pair = false;
        let mut two_pairs = false;
        let mut toak = false;
        let joker_count: u32 = count_by_type[0];

        if joker_count >= 4 {
            insert_into(&mut hands[FIVE_OF_A_KIND], hand, value);
            continue 'line;
        }
        for i in 1..count_by_type.len() {
            if joker_count + count_by_type[i] == 5 {
                insert_into(&mut hands[FIVE_OF_A_KIND], hand, value);
                continue 'line;
            }
            if joker_count + count_by_type[i] == 4 {
                insert_into(&mut hands[FOUR_OF_A_KIND], hand, value);
                continue 'line;
            }
            if count_by_type[i] == 3 {
                if pair {
                    insert_into(&mut hands[FULL_HOUSE], hand, value);
                    continue 'line;
                }
                toak = true;
            }
            if count_by_type[i] == 2 {
                if toak {
                    insert_into(&mut hands[FULL_HOUSE], hand, value);
                    continue 'line;
                }
                if pair {
                    two_pairs = true;
                }
                pair = true;
            }
        }
        if two_pairs {
            if joker_count == 1 {
                insert_into(&mut hands[FULL_HOUSE], hand, value);
                continue 'line;
            }
            insert_into(&mut hands[TWO_PAIRS], hand, value);
            continue 'line;
        }
        if toak || pair as u32 + joker_count == 2 {
            insert_into(&mut hands[THREE_OF_A_KIND], hand, value);
            continue 'line;
        }
        if pair || joker_count == 1 {
            insert_into(&mut hands[ONE_PAIR], hand, value);
            continue 'line;
        }
        insert_into(&mut hands[HIGH_CARD], hand, value);
    }

    let mut position = 1;
    for hands in hands {
        for first_card in hands {
            for hand in first_card {
                result += hand.1 * position;
                position += 1;
            }
        }
    }

    result.into()
}
