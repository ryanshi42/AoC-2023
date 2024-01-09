use std::{cmp::max, collections::HashSet, sync::atomic::AtomicU64, panic::AssertUnwindSafe};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Up,
    Down,
    Left,
    Right,
    Wall,
}

impl Tile {
    fn forward(&self) -> Self {
        match self {
            Tile::Empty => Tile::Up,
            Tile::Up => Tile::Down,
            Tile::Down => Tile::Left,
            Tile::Left => Tile::Right,
            Tile::Right => Tile::Empty,
            Tile::Wall => Tile::Wall,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '>' => Tile::Right,
                    'v' => Tile::Down,
                    '<' => Tile::Left,
                    '^' => Tile::Up,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

impl Coord {
    fn neighbours(&self, map: &Vec<Vec<Tile>>, seen: &HashSet<Coord>) -> Vec<Coord> {
        let rows = map.len();
        let cols = map[0].len();
        let mut res = Vec::new();

        match map[self.y][self.x] {
            Tile::Empty => (),
            Tile::Up => {
                if self.y > 0 && !seen.contains(&Coord { x: self.x, y: self.y - 1 }) {
                    res.push(Coord { x: self.x, y: self.y - 1 });
                }
                return res;
            },
            Tile::Down => {
                if self.y < rows - 1 && !seen.contains(&Coord { x: self.x, y: self.y + 1 }) {
                    res.push(Coord { x: self.x, y: self.y + 1 });
                }
                return res; 
            },
            Tile::Left => { 
                if self.x > 0 && !seen.contains(&Coord { x: self.x - 1, y: self.y }) {
                    res.push(Coord { x: self.x - 1, y: self.y });
                }
                return res;
            },
            Tile::Right => {
                if self.x < cols - 1 && !seen.contains(&Coord { x: self.x + 1, y: self.y }) {
                    res.push(Coord { x: self.x + 1, y: self.y });
                }
                return res; 
            },
            Tile::Wall => panic!("at the disco"),
        }

        // Left
        if self.x > 0 && self.y != 0 && self.y != cols - 1 {
            if !seen.contains(&Coord { x: self.x - 1, y: self.y }) && (map[self.y][self.x - 1] == Tile::Empty || map[self.y][self.x - 1] == Tile::Left) {
                res.push(Coord { x: self.x - 1, y: self.y });
            }
        }
        if self.x < cols - 1 {
            if !seen.contains(&Coord { x: self.x + 1, y: self.y }) && (map[self.y][self.x + 1] == Tile::Empty || map[self.y][self.x + 1] == Tile::Right) {
                res.push(Coord { x: self.x + 1, y: self.y });
            }
        }
        // Up
        if self.y > 0 && self.x != 0 && self.x != rows - 1 {
            if !seen.contains(&Coord { x: self.x, y: self.y - 1 }) && (map[self.y - 1][self.x] == Tile::Empty || map[self.y - 1][self.x] == Tile::Up) {
                res.push(Coord { x: self.x, y: self.y - 1 });
            }
        }
        if self.y < rows - 1 {
            if !seen.contains(&Coord { x: self.x, y: self.y + 1 }) && (map[self.y + 1][self.x] == Tile::Empty || map[self.y + 1][self.x] == Tile::Down) {
                res.push(Coord { x: self.x, y: self.y + 1 });
            }
        }

        res
    }
}

fn dfs(map: &Vec<Vec<Tile>>, mut seen: HashSet<Coord>, next: &Coord, end: &Coord, width: &usize, height: &usize) -> usize {
    seen.insert(*next);
    if next == end {
        return seen.len() - 1;
    }

    let mut res = vec![];
    for nbhr in next.neighbours(map, &seen) {
        res.push(dfs(map, seen.clone(), &nbhr, end, width, height));
    }
    match res.iter().max() {
        Some(max) => *max,
        None => 0,
    }
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let map = parse(input);
    let start = Coord { x: 1, y: 0 };
    let end = Coord { x: map[0].len() - 2, y: map.len() - 1 };
    let seen = HashSet::new();

    let width = map[0].len();
    let height = map.len();
    dfs(&map, seen, &start, &end, &width, &height)
}


fn parse2(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '>' => Tile::Empty,
                    'v' => Tile::Empty,
                    '<' => Tile::Empty,
                    '^' => Tile::Empty,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    let map = parse2(input);
    let start = Coord { x: 1, y: 0 };
    let end = Coord { x: map[0].len() - 2, y: map.len() - 1 };
    let seen = HashSet::new();

    let width = map[0].len();
    let height = map.len();
    dfs(&map, seen, &start, &end, &width, &height)
}