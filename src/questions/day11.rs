const X_SIZE: usize = 140;
const Y_SIZE: usize = 140;


pub fn part_one(input: &str) -> i128 {
    let mut result: u64 = 0;
    let bytes = input.as_bytes();
    let mut collumns: [u32; X_SIZE] = [0; X_SIZE];
    let mut rows: [u32; Y_SIZE] = [0; Y_SIZE];

    for i in 0..Y_SIZE {
        for j in 0..X_SIZE {
            if bytes[i*(X_SIZE+1) + j] == b'#' {
                rows[i] += 1;
                collumns[j] += 1;
            }
        }
    }

    let mut acc = collumns[0] as u64;
    let mut gallaxy_count = acc;
    for collumn_count in &collumns[1..] {
        result += acc**collumn_count as u64;
        if *collumn_count == 0 {
            acc += 2*gallaxy_count;
        } else {
            gallaxy_count += *collumn_count as u64;
            acc += gallaxy_count;
        }
    }

    let mut acc = rows[0] as u64;
    let mut gallaxy_count = acc;
    for row_count in &rows[1..] {
        result += acc**row_count as u64;
        if *row_count == 0 {
            acc += 2*gallaxy_count;
        } else {
            gallaxy_count += *row_count as u64;
            acc += gallaxy_count;
        }
    }
    
    result.into()
}

pub fn part_two(input: &str) -> i128 {
    let mut result: u64 = 0;
    let bytes = input.as_bytes();
    let mut collumns: [u32; X_SIZE] = [0; X_SIZE];
    let mut rows: [u32; Y_SIZE] = [0; Y_SIZE];

    for i in 0..Y_SIZE {
        for j in 0..X_SIZE {
            if bytes[i*(X_SIZE+1) + j] == b'#' {
                rows[i] += 1;
                collumns[j] += 1;
            }
        }
    }

    let mut acc = collumns[0] as u64;
    let mut gallaxy_count = acc;
    for collumn_count in &collumns[1..] {
        result += acc**collumn_count as u64;
        if *collumn_count == 0 {
            acc += 1_000_000*gallaxy_count;
        } else {
            gallaxy_count += *collumn_count as u64;
            acc += gallaxy_count;
        }
    }

    let mut acc = rows[0] as u64;
    let mut gallaxy_count = acc;
    for row_count in &rows[1..] {
        result += acc**row_count as u64;
        if *row_count == 0 {
            acc += 1_000_000*gallaxy_count;
        } else {
            gallaxy_count += *row_count as u64;
            acc += gallaxy_count;
        }
    }
    
    result.into()
}