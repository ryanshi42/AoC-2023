use std::{collections::{HashSet}};

// Credits to https://nickymeuleman.netlify.app/garden/aoc2023-day16 for the initial setup.
// Use enum Tile implementation whenever you have different behaviours!!
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    SplitHoriz,
    SplitVert,
    MirrorForward,
    MirrorBack,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Beam {
    pos: Coord,
    dir: Direction,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Note for future self: useful derives to have.
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]

// Implement a method which given a Beam, combine two if statements to get new coords
impl Beam {
    fn forward(mut self, rows: usize, cols: usize) -> Option<Self> {
        match self.dir {
            Direction::Up if self.pos.y > 0 => self.pos.y -= 1,
            Direction::Down if self.pos.y < rows - 1 => self.pos.y += 1,
            Direction::Left if self.pos.x > 0 => self.pos.x -= 1,
            Direction::Right if self.pos.x < cols - 1 => self.pos.x += 1,
            _ => return None,
        }
        Some(self)
    }
}

fn parse(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    // Watch out for the escape character here
                    '\\' => Tile::MirrorBack,
                    '/' => Tile::MirrorForward,
                    '.' => Tile::Empty,
                    '-' => Tile::SplitHoriz,
                    '|' => Tile::SplitVert,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

fn f(start: Beam, map: &Vec<Vec<Tile>>) -> usize {
    let mut stack = vec![];
    stack.push(start);

    let mut energized = HashSet::new();
    let mut seen = HashSet::new();

    // let mut seen = HashMap::new();
    // While let Some is much better syntax than using unwrap
    while let Some(mut beam) = stack.pop() {
        // Avoid infinite light loops
        if seen.contains(&beam) {
            continue;
        }
        energized.insert(beam.pos);
        seen.insert(beam);
        let dirs = match (map[beam.pos.y][beam.pos.x], beam.dir) {
            (Tile::Empty, _)
            | (Tile::SplitHoriz, Direction::Left)
            | (Tile::SplitHoriz, Direction::Right)
            | (Tile::SplitVert, Direction::Up)
            | (Tile::SplitVert, Direction::Down) => vec![beam.dir],
            (Tile::SplitHoriz, _) => {
                vec![Direction::Left, Direction::Right]
            }
            (Tile::SplitVert, _) => {
                vec![Direction::Up, Direction::Down]
            }
            (Tile::MirrorForward, Direction::Up) | (Tile::MirrorBack, Direction::Down) => {
                vec![Direction::Right]
            }
            (Tile::MirrorForward, Direction::Down) | (Tile::MirrorBack, Direction::Up) => {
                vec![Direction::Left]
            }
            (Tile::MirrorForward, Direction::Left) | (Tile::MirrorBack, Direction::Right) => {
                vec![Direction::Down]
            }
            (Tile::MirrorForward, Direction::Right) | (Tile::MirrorBack, Direction::Left) => {
                vec![Direction::Up]
            }
        };
        for dir in dirs {
            beam.dir = dir;
            if let Some(beam) = beam.forward(map.len(), map[0].len()) {
                stack.push(beam);
            }
        }
    }
    energized.len() 
    
}

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let map = parse(input); 
    f(Beam {
        dir: Direction::Right,
        pos: Coord { x: 0, y: 0 },
    }, &map)
}

// Brute force approach. This was very trek so I just copied again from the source above. Thanks! xx
#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
    let grid = parse(input);
    let from_left = (0..grid.len()).map(|row| Beam {
        dir: Direction::Right,
        pos: Coord { x: 0, y: row },
    });
    let from_right = (0..grid.len()).map(|row| Beam {
        dir: Direction::Left,
        pos: Coord {
            x: grid[0].len() - 1,
            y: row,
        },
    });
    let from_up = (0..grid[0].len()).map(|col| Beam {
        dir: Direction::Down,
        pos: Coord { x: col, y: 0 },
    });
    let from_down = (0..grid[0].len()).map(|col| Beam {
        dir: Direction::Up,
        pos: Coord {
            x: col,
            y: grid.len() - 1,
        },
    });

    from_left
        .chain(from_right)
        .chain(from_up)
        .chain(from_down)
        .map(|start| f(start, &grid))
        .max()
        .unwrap()
}