use std::collections::{VecDeque, HashSet};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

fn parse(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => Tile::Start,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

fn get_nbhrs(curr: Coord, width: usize, height: usize, map: &Vec<Vec<Tile>>) -> Vec<Coord> {
    let mut ret = vec![];
    let Coord { x: ci, y: cj } = curr;
    for (ti, tj) in [(ci + 1, cj), (ci - 1, cj), (ci, cj + 1), (ci, cj - 1)] {
        if map[tj as usize % height][ti as usize % width] == Tile::Empty {
            ret.push(Coord { x: ti, y: tj });
        }
        // if 0 <= ti && ti < width as i64 && 0 <= tj && tj < height as i64 && map[ti as usize][ti as usize] == Tile::Empty {
        //     ret.push(Coord { x: ti, y: tj });
        // }
    }
    ret
}

fn bfs(map: Vec<Vec<Tile>>, start: Coord, max: usize) -> usize {
    let mut q = VecDeque::new(); 
    q.push_back((start, 0));
    let mut seen = HashSet::new(); 
    let width = map[0].len();
    let height = map.len();

    let mut ans = 0;

    while !q.is_empty() {
        let (curr, stage) = q.pop_front().unwrap();

        if seen.get(&curr).is_some() || stage > max {
            continue;
        }
        if stage % 2 == 0 {
            ans += 1;
        }
        seen.insert(curr);
        for nbhr in get_nbhrs(curr, width, height, &map) {
            if seen.get(&nbhr).is_none() {
                q.push_back((nbhr, stage + 1));
            } 
        }
    }
    ans
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut start = Coord { x: 0, y: 0 };
    for (i, row) in map.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == Tile::Start {
                start = Coord { x: j as i64, y: i as i64 };
                break;
            }
        }
    }
    bfs(map, start, 64)
}

// Brute force approach. This was very trek so I just copied again from the source above. Thanks! xx
#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    // should chain
    let map = parse(input);
    let mut start = Coord { x: 0, y: 0 };
    for (i, row) in map.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == Tile::Start {
                start = Coord { x: j as i64, y: i as i64 };
                break;
            }
        }
    }
    bfs(map, start, 26501365)
}