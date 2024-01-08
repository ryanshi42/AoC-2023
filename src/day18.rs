use core::panic;
use std::ops::Div;



#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Instr<'a> {
    dir: Dir,
    num: i64,
    hex: &'a str,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn advance(&self, direction: &Dir, amount: i64) -> Self {
        match direction {
            Dir::Up => Self {
                x: self.x + amount,
                y: self.y,
            },
            Dir::Down => Self {
                x: self.x - amount,
                y: self.y,
            },
            Dir::Left => Self {
                x: self.x,
                y: self.y - amount,
            },
            Dir::Right => Self {
                x: self.x,
                y: self.y + amount,
            },
        }
    }
}


fn to_num(hex: char) -> i64 {
    // println!("{hex}");
    match hex {
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => hex.to_string().parse().unwrap()
    }
}

impl<'a> Instr<'a> {
    fn new(line: &'a str) -> Self {
        let Some((instr, hex)) = line.split_once(" (") else { panic!("hi") };
        let Some((dir, new_num)) = instr.split_once(" ") else {panic!("hi") };
        let new_hex = &hex[0..hex.len()];
        Instr {
            dir: match dir {
                "U" => Dir::Up,
                "L" => Dir::Left,
                "R" => Dir::Right,
                "D" => Dir::Down,
                _ => panic!("at the disco"),
            },
            num: new_num.parse::<i64>().unwrap(),
            hex: new_hex
        }
    }

    fn new2(line: &'a str) -> Self {
        let Some((instr, hex)) = line.split_once(" (") else { panic!("hi") };
        let Some((_dir, _new_num)) = instr.split_once(" ") else {panic!("hi") };
        let new_hex = &hex[1..hex.len()];
        let dir = new_hex.chars().nth(5).unwrap(); 
        let new_num = new_hex.chars().enumerate().take(5).fold(0, |acc, (i, x)| acc + (16_usize.pow((4 - i).try_into().unwrap())) * to_num(x) as usize);
        // println!("{dir:?} {new_num}");
        Instr {
            dir: match dir {
                '3' => Dir::Up,
                '2' => Dir::Left,
                '0' => Dir::Right,
                '1' => Dir::Down,
                _ => panic!("at the disco"),
            },
            num: new_num as i64,
            hex: new_hex
        }
    }
}

fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(|line| Instr::new(line)).collect() 
}

fn parse2(input: &str) -> Vec<Instr> {
    input.lines().map(|line| Instr::new2(line)).collect() 
}

fn calc_area<'a>(vv: impl Iterator<Item = Instr<'a>>) -> i64 {
    let (area, perim, _) = vv.fold(
        (0, 0, Coord { x: 0, y: 0 }),
        |(ca, cper, cpos), Instr { dir, num, hex: _ }| {
            let new_pos = cpos.advance(&dir, num);
            let new_area = ca + (cpos.x * new_pos.y - cpos.y * new_pos.x);
            let new_perim = cper + num; 
            (new_area, new_perim, new_pos) 
        }
    );

    // Todo, prove this.
    // Impls, parse, new, impl iterator, fold syntax (curr, function next, picks formula + thingo)
    return area.div(2) + perim.div(2) + 1;
}

#[aoc(day18, part1)]
fn part1(input: &str) -> i64 {
    
    // use Pick's formula + shoelace formula
    let instrs = parse(input).into_iter();

    calc_area(instrs)
}

// Brute force approach. This was very trek so I just copied again from the source above. Thanks! xx
#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {
    // should chain
    let instrs = parse2(input).into_iter();

    calc_area(instrs)
}