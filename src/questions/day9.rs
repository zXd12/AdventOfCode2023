const NUMBER_PER_LINE: usize = 21;

pub fn part_one(input: &str) -> i128 {
    let mut result: i64 = 0;
    
    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut numbers: [i64; NUMBER_PER_LINE] = Default::default();
        let mut number_index = 0;
        let mut neg = false;
        let mut acc = 0;
        for byte in bytes {
            match byte {
                b' ' => {
                    if neg {
                        numbers[number_index] = -(acc as i64);
                    } else {
                        numbers[number_index] = acc as i64;
                    }
                    number_index += 1;
                    neg = false;
                    acc = 0;
                },
                b'-' => neg = true,
                value => acc = acc * 10 + (value - b'0') as u32,
            }
        }
        result += numbers[NUMBER_PER_LINE - 1];
        let mut iter_count = 1;

        loop {
            let mut constant = true;
            let first_num = numbers[1] - numbers[0];
            numbers[0] = first_num;
            for number_index in 1..NUMBER_PER_LINE - iter_count {
                let new_number = numbers[number_index + 1] - numbers[number_index];
                constant &= new_number == first_num;
                numbers[number_index] = new_number;
            }
            iter_count += 1;
            result += numbers[NUMBER_PER_LINE - iter_count];
            if constant {
                break;
            }
        }
    }

    result.into()
}

pub fn part_two(input: &str) -> i128 {
    let mut result: i64 = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut numbers: [i64; NUMBER_PER_LINE] = Default::default();
        let mut number_index = 0;
        let mut neg = false;
        let mut acc = 0;
        for byte in bytes {
            match byte {
                b' ' => {
                    if neg {
                        numbers[number_index] = -(acc as i64);
                    } else {
                        numbers[number_index] = acc as i64;
                    }
                    number_index += 1;
                    neg = false;
                    acc = 0;
                },
                b'-' => neg = true,
                value => acc = acc * 10 + (value - b'0') as u32,
            }
        }

        let mut iter_count = 1;
        result += numbers[0];
        neg = true;

        loop {
            let mut constant = true;
            let first_num = numbers[1] - numbers[0];
            numbers[0] = first_num;
            for number_index in 1..NUMBER_PER_LINE - iter_count {
                let new_number = numbers[number_index + 1] - numbers[number_index];
                constant &= new_number == first_num;
                numbers[number_index] = new_number;
            }
            iter_count += 1;
            if neg {
                result -= first_num
            } else {
                result += first_num
            }
            if constant {
                break;
            }
            neg = !neg;
        }
    }

    result as i128
}
