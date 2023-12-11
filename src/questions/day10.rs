// const LINE: usize = 141;
const LINE: usize = 6;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn follow_pipe(bytes: &[u8], pos: &mut usize, direction: &mut Direction) -> bool {
    match direction {
        Direction::Up => {
            *pos -= LINE;
            *direction = match bytes[*pos] {
                b'|' => Direction::Up,
                b'F' => Direction::Right,
                b'7' => Direction::Left,
                _ => return false
            };
        },
        Direction::Right => {
            *pos += 1;
            *direction = match bytes[*pos] {
                b'J' => Direction::Up,
                b'-' => Direction::Right,
                b'7' => Direction::Down,
                _ => return false
            };
        },
        Direction::Down => {
            *pos += LINE;
            *direction = match bytes[*pos] {
                b'L' => Direction::Right,
                b'|' => Direction::Down,
                b'J' => Direction::Left,
                _ => return false
            };
        },
        Direction::Left => {
            *pos -= 1;
            *direction = match bytes[*pos] {
                b'L' => Direction::Up,
                b'F' => Direction::Down,
                b'-' => Direction::Left,
                _ => return false
            };
        },
    }
    true
}

pub fn part_one(input: &str) -> i128 {

    let bytes = input.as_bytes();
    let mut pos: usize = 0;

    for i in 0..bytes.len() {
        if bytes[i] == b'S' {
            pos = i;
            break;
        }
    }

    let mut direction: Direction = Direction::Up;

    if !follow_pipe(bytes, &mut pos, &mut direction) {
        pos += LINE;
        direction = Direction::Right;
        if !follow_pipe(bytes, &mut pos, &mut direction) {
            pos -= 1;
            direction = Direction::Down;
            follow_pipe(bytes, &mut pos, &mut direction);
        }
    }

    let mut pipe_length = 2;
    while follow_pipe(bytes, &mut pos, &mut direction) {
        pipe_length += 1;
    }

    return (pipe_length/2).into();

}

struct MapData<'a> {
    bytes: &'a [u8],
    pos: usize,
    direction: Direction,
    area: isize,
}

impl<'a> MapData<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            pos: 0,
            direction: Direction::Up,
            area: 0,
        }
    }
    
    fn follow_pipe(&mut self) -> bool {
        match self.direction {
            Direction::Up => {
                self.area -= 1;
                self.pos -= LINE;
                self.direction = match self.bytes[self.pos] {
                    b'|' => Direction::Up,
                    b'F' => Direction::Right,
                    b'7' => Direction::Left,
                    _ => return false
                };
            },
            Direction::Right => {
                self.area += (self.pos/LINE) as isize;
                self.pos += 1;
                self.direction = match self.bytes[self.pos] {
                    b'J' => Direction::Up,
                    b'-' => Direction::Right,
                    b'7' => Direction::Down,
                    _ => return false
                };
            },
            Direction::Down => {
                self.pos += LINE;
                self.direction = match self.bytes[self.pos] {
                    b'L' => Direction::Right,
                    b'|' => Direction::Down,
                    b'J' => Direction::Left,
                    _ => return false
                };
            },
            Direction::Left => {
                self.area -= (self.pos/LINE) as isize + 1;
                self.pos -= 1;
                self.direction = match self.bytes[self.pos] {
                    b'L' => Direction::Up,
                    b'F' => Direction::Down,
                    b'-' => Direction::Left,
                    _ => return false
                };
            },
        }
        true
    }
}

pub fn part_two(input: &str) -> i128 {

    let bytes = input.as_bytes();
    let mut map = MapData::new(bytes);

    for i in 0..bytes.len() {
        if map.bytes[i] == b'S' {
            map.pos = i;
            break;
        }
    }

    if !map.follow_pipe() {
        map.area += 1;
        map.pos += LINE;
        map.direction = Direction::Right;
        if !map.follow_pipe() {
            map.area -= (map.pos/LINE) as isize;
            map.pos -= 1;
            map.direction = Direction::Down;
            map.follow_pipe();
        }
    }
    println!("{}", map.area);

    while map.follow_pipe() {
        println!("{}", map.area);
    }

    return (map.area) as i128;

}