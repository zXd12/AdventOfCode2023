use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Square {
    Number(u32),
    Pointer(usize),
    Other,
}

#[derive(Debug)]
struct Window {
    line_numbers: VecDeque<Vec<Square>>,
    line_symbols: VecDeque<Vec<usize>>,
}

impl Window {
    pub fn new(line_length: usize) -> Self {
        Self {
            line_numbers: vec![vec![Square::Other; line_length]; 3].into_iter().collect(),
            line_symbols: vec![Vec::new(); 2].into_iter().collect(),
        }
    }

    fn get_numbers_arround_part(&mut self, line_index: usize, part_index: usize) -> u32 {
        let mut result = 0;
        match self.line_numbers[line_index][part_index-1] {
            Square::Number(value) => {
                self.line_numbers[line_index][part_index-1] = Square::Other;
                result += value;
                if let Square::Number(value) = self.line_numbers[line_index][part_index+1] {
                    self.line_numbers[line_index][part_index+1] = Square::Other;
                    result += value;
                }
            },
            Square::Pointer(index) => { 
                if let Square::Number(value) = self.line_numbers[line_index][index] {
                    self.line_numbers[line_index][index] = Square::Other;
                    result += value;
                    if let Square::Number(value) = self.line_numbers[line_index][part_index+1] {
                        self.line_numbers[line_index][part_index+1] = Square::Other;
                        result += value;
                    }
                }
             },
            _ => {
                if let Square::Number(value) = self.line_numbers[line_index][part_index] {
                    self.line_numbers[line_index][part_index] = Square::Other;
                    result += value;
                } else if let Square::Number(value) = self.line_numbers[line_index][part_index+1] {
                    self.line_numbers[line_index][part_index+1] = Square::Other;
                    result += value;
                }
            },
        }

        result
    }


    pub fn compute_total_parts_values(&mut self) -> u32 {
        let mut result = 0;
        let symbols = &self.line_symbols[0].clone();
        for symbole_pos in symbols {
            result += self.get_numbers_arround_part(0, *symbole_pos);
            result += self.get_numbers_arround_part(1, *symbole_pos);
            result += self.get_numbers_arround_part(2, *symbole_pos);
        }
        result
    }

    fn get_numbers_adjacent_to_gear(&self, adjacent_numbers: &mut Vec<u32>, line_index: usize, gear_index: usize) {
        match self.line_numbers[line_index][gear_index-1] {
            Square::Number(value) => {
                adjacent_numbers.push(value);
                if let Square::Number(value) = self.line_numbers[line_index][gear_index+1] {
                    adjacent_numbers.push(value);
                }
            },
            Square::Pointer(index) => { 
                if let Square::Number(value) = self.line_numbers[line_index][index] {
                    adjacent_numbers.push(value);
                    if let Square::Number(value) = self.line_numbers[line_index][gear_index+1] {
                        adjacent_numbers.push(value);
                    }
                }
             },
            _ => {
                if let Square::Number(value) = self.line_numbers[line_index][gear_index] {
                    adjacent_numbers.push(value);
                } else if let Square::Number(value) = self.line_numbers[line_index][gear_index+1] {
                    adjacent_numbers.push(value);
                }
            },
        }
    }

    pub fn compute_gear_ratios(&self) -> u32 {
        let mut result = 0;

        for symbole_pos in &self.line_symbols[1] {
            let mut adjacent_numbers: Vec<u32> = Vec::new();

            match self.line_numbers[1][symbole_pos-1] {
                Square::Number(value) => adjacent_numbers.push(value),
                Square::Pointer(index) => { 
                    if let Square::Number(value) = self.line_numbers[1][index] {
                        adjacent_numbers.push(value);
                    }
                 },
                _ => {},
            }

            match self.line_numbers[1][symbole_pos+1] {
                Square::Number(value) => adjacent_numbers.push(value),
                _ => {},
            }

            self.get_numbers_adjacent_to_gear(&mut adjacent_numbers, 0, *symbole_pos);
            self.get_numbers_adjacent_to_gear(&mut adjacent_numbers, 2, *symbole_pos);

            if adjacent_numbers.len() == 2 {
                result += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }

        result
    }

    pub fn new_line(&mut self) {
        self.line_numbers.pop_back();
        self.line_numbers.push_front(vec![Square::Other; self.line_numbers[0].len()]);

        self.line_symbols.pop_back();
        self.line_symbols.push_front(Vec::new());
    }
}

pub fn part_one(input: &str) -> i128 {
    let mut result = 0;

    let mut window = Window::new(140);

    for line in input.lines() {
        let mut index = 0;
        let bytes  = line.as_bytes();

        while index < 140 {
            if bytes[index] == b'.' {
                index += 1;
                continue;
            }
            if bytes[index] >= b'0' && bytes[index] <= b'9' {
                let mut number = (bytes[index] - b'0') as u32;
                let mut number_len = 1;
                loop {
                    if index + number_len >= 140 || bytes[index + number_len] < b'0' || bytes[index + number_len] > b'9' {
                        break;
                    }
                    number = number * 10 + (bytes[index + number_len] - b'0') as u32;
                    window.line_numbers[0][index + number_len] = Square::Pointer(index);
                    number_len += 1;
                }
                window.line_numbers[0][index] = Square::Number(number);
                index += number_len;
                continue;
            }
            window.line_symbols[0].push(index);
            index += 1;
        }

        result += window.compute_total_parts_values();
        window.new_line();
    }

    result += window.compute_total_parts_values();

    result.into()
}

pub fn part_two(input: &str) -> i128 {
    let mut result = 0;

    let mut window = Window::new(140);

    for line in input.lines() {
        let mut index = 0;
        let bytes  = line.as_bytes();

        while index < 140 {
            if bytes[index] == b'*' {
                window.line_symbols[0].push(index);
                index += 1;
                continue;
            }
            if bytes[index] >= b'0' && bytes[index] <= b'9' {
                let mut number = (bytes[index] - b'0') as u32;
                let mut number_len = 1;
                loop {
                    if index + number_len >= 140 || bytes[index + number_len] < b'0' || bytes[index + number_len] > b'9' {
                        break;
                    }
                    number = number * 10 + (bytes[index + number_len] - b'0') as u32;
                    window.line_numbers[0][index + number_len] = Square::Pointer(index);
                    number_len += 1;
                }
                window.line_numbers[0][index] = Square::Number(number);
                index += number_len;
                continue;
            }
            index += 1;
        }

        result += window.compute_gear_ratios();
        window.new_line();
    }

    result += window.compute_gear_ratios();

    result.into()
}
