pub fn part_one(input: &str) -> u64 {
    let mut result: u32 = 0;

    for line in input.lines() {
        let mut card_score = 1;

        let line_bytes = line.as_bytes();
        let mut winning_table = [[false; 10]; 10];

        for i in (10..39).step_by(3) {
            winning_table[if line_bytes[i] == 32 {0} else {line_bytes[i]- 48} as usize][(line_bytes[i + 1] - 48) as usize] = true;
        }

        for i in (42..117).step_by(3) {
            if winning_table[if line_bytes[i] == 32 {0} else {line_bytes[i]- 48} as usize][(line_bytes[i + 1] - 48) as usize] {
                card_score <<= 1;
            }
        }
        result += card_score >> 1;
    }

    result.into()
}

pub fn part_two(input: &str) -> u64 {
    let mut result: u32 = 0;
    let mut card_duplicates = vec![];
    card_duplicates.resize(input.lines().count(), 1);

    for (i, line) in input.lines().enumerate() {
        let mut card_score = 0;

        let line_bytes = line.as_bytes();
        let mut winning_table = [[false; 10]; 10];

        for i in (10..39).step_by(3) {
            winning_table[if line_bytes[i] == 32 {0} else {line_bytes[i]- 48} as usize][(line_bytes[i + 1] - 48) as usize] = true;
        }

        for i in (42..117).step_by(3) {
            if winning_table[if line_bytes[i] == 32 {0} else {line_bytes[i]- 48} as usize][(line_bytes[i + 1] - 48) as usize] {
                card_score += 1;
            }
        }

        for j in 1..card_score + 1 {
            if i + j < card_duplicates.len() {
                card_duplicates[i + j] += card_duplicates[i];
            }
        }

        result += card_duplicates[i];
    }

    result.into()
}
