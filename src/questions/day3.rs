use std::{io::{self, BufRead, BufReader}, fs::File};

#[derive(Clone, Copy, Debug)]
struct Number {
    pub value: u32,
    start_pos: usize,
    end_pos: usize,
}

impl Number {
    pub fn is_adjacent(&self, pos: usize) -> bool {
        return self.end_pos + 1 >= pos && pos >= self.start_pos.saturating_sub(1);
    }

    pub fn add_digit(&mut self, digit: char) {
        self.value *= 10;
        self.value += digit.to_digit(10).unwrap();
        self.end_pos += 1;
    }
}

#[derive(Debug)]
struct Lines {
    first_line_numbers: Vec<Number>,
    middle_line_numbers: Vec<Number>,
    middle_line_symbols: Vec<usize>,
    last_line_numbers: Vec<Number>,
    last_line_symbols: Vec<usize>,
    current_number: Option<Number>,
}

impl Lines {
    pub fn new() -> Self {
        Self {
            first_line_numbers: vec![],
            middle_line_numbers: vec![],
            last_line_numbers: vec![],
            current_number: None,
            middle_line_symbols: vec![],
            last_line_symbols: vec![],
        }
    }

    pub fn add_digit(&mut self, index: usize, digit: char) {

        match self.current_number {
            None => {
                self.current_number = Some(Number {
                    value: digit.to_digit(10).unwrap(),
                    start_pos: index,
                    end_pos: index,
                })
            }
            Some(ref mut number) => {
                number.add_digit(digit);
            },
        }
    }

    pub fn end_current_number(&mut self) {
        if self.current_number.is_some() {
            self.last_line_numbers.push(self.current_number.unwrap());
            self.current_number = None;
        }
    }

    pub fn add_symbol(&mut self, index: usize) {
        self.last_line_symbols.push(index);
    }

    fn compute_line_parts_values(line: &Vec<Number>, symbole_pos: usize, result: &mut u32) -> Vec<Number> {
        let mut new_line = vec![];
        for number in line.into_iter() {
            if number.is_adjacent(symbole_pos) {
                *result += number.value;
            } else {
                new_line.push(number.clone());
            }
        }
        new_line
    }

    pub fn compute_total_parts_values(&mut self) -> u32 {
        let mut result = 0;
        for symbole_pos in &self.middle_line_symbols {
            self.first_line_numbers = Self::compute_line_parts_values(&self.first_line_numbers, *symbole_pos, &mut result);
            self.middle_line_numbers = Self::compute_line_parts_values(&self.middle_line_numbers, *symbole_pos, &mut result);
            self.last_line_numbers = Self::compute_line_parts_values(&self.last_line_numbers, *symbole_pos, &mut result);
        }
        result
    }

    pub fn compute_gear_ratios(&self) -> u32 {
        let mut result = 0;
        let mut all_lines = vec![];
        all_lines.extend(self.first_line_numbers.clone());
        all_lines.extend(self.middle_line_numbers.clone());
        all_lines.extend(self.last_line_numbers.clone());

        for gear in &self.middle_line_symbols {
            let mut adjacent_number_count = 0;
            let mut adjacent_number_power = 1;
            for number in &all_lines {
                if number.is_adjacent(*gear) {
                    adjacent_number_count += 1;
                    adjacent_number_power *= number.value;
                }
                if adjacent_number_count >= 2 {
                    break;
                }
            }
            if adjacent_number_count == 2 {
                result += adjacent_number_power;
            }
        }

        result
    }

    pub fn new_line(&mut self) {
        self.first_line_numbers = self.middle_line_numbers.clone();
        self.middle_line_numbers = self.last_line_numbers.clone();
        self.last_line_numbers = vec![];

        self.middle_line_symbols = self.last_line_symbols.clone();
        self.last_line_symbols = vec![];
    }
}

pub fn part_one(reader: BufReader<File>) -> io::Result<u32> {

    let mut result = 0;

    let mut lines = Lines::new();

    for line in reader.lines() {
        let line = line?;

        for (i, char) in line.chars().enumerate() {
            match char {
                ch if ch.is_digit(10) => {
                    lines.add_digit(i, char);
                }
                '.' => lines.end_current_number(),
                _ => {
                    lines.end_current_number();
                    lines.add_symbol(i);
                },
            }
        }

        lines.end_current_number();
        result += lines.compute_total_parts_values();
        lines.new_line();
    }

    result += lines.compute_total_parts_values();

    Ok(result)
}

pub fn part_two(reader: BufReader<File>) -> io::Result<u32> {

    let mut result = 0;

    let mut lines = Lines::new();

    for line in reader.lines() {
        let line = line?;

        for (i, char) in line.chars().enumerate() {
            match char {
                ch if ch.is_digit(10) => {
                    lines.add_digit(i, char);
                }
                '*' => {
                    lines.end_current_number();
                    lines.add_symbol(i);
                },
                _ => lines.end_current_number(),
            }
        }

        lines.end_current_number();
        result += lines.compute_gear_ratios();
        lines.new_line();
    }

    result += lines.compute_gear_ratios();

    Ok(result)
}
