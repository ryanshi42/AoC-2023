
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Up,
    Down,
    Left,
    Right,
    Wall,
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

fn dfs(map: Vec<Vec<Tile>>, start: Coord, max: usize) -> usize {
    let mut stack = vec![]; 
    stack.push((start, 0));

    let width = map[0].len();
    let height = map.len();

    let SHADOW_ROW = [Tile::Empty; width];
    let mut shadow_grid = [SHADOW_ROW; height];

    let mut ans = 0;

    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        ans = max(ans, stack.len());
        match map[curr.j][curr.i] {

        }

        let curr_stage = shadow_grid[curr.j][curr.i]; 
        if curr_stage == Tile::Wall || curr_stage == Tile::Right {
            continue;
        }
        // How to do this in Rust?
        let next_stage = curr_stage + 1;
        match next_stage {

        }

    }
    ans
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut start = Coord { x: 0, y: 0 };
    for (j, elem) in map[0].iter().enumerate() {
        if elem == '.' {
            start = Coord { x: j as i64, y: 0 as i64 };
            break;
        }
    }
    dfs(map, start)
}

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    // should chain
    0
}