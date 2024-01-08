use std::{collections::{BinaryHeap, HashSet}};

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

fn get_nbhrs(curr: Coord, width: i64, height: i64, dir: Direction, numindir: usize) -> Vec<(Coord, Direction, usize)> {
    let mut ret = Vec::new();
    let Coord { x: ci, y: cj } = curr;
    for (ti, tj, d) in [(ci + 1, cj, Direction::Right), (ci - 1, cj, Direction::Left), (ci, cj + 1, Direction::Down), (ci, cj - 1, Direction::Up)] {
        let mut new_num = 1;
        match (dir, d) {
            (Direction::Left, Direction::Right) => continue,
            (Direction::Right, Direction::Left) => continue,
            (Direction::Up, Direction::Down) => continue,
            (Direction::Down, Direction::Up) => continue,
            (Direction::Left, Direction::Left) => new_num = numindir + 1,
            (Direction::Right, Direction::Right) => new_num = numindir + 1,
            (Direction::Up, Direction::Up) => new_num = numindir + 1,
            (Direction::Down, Direction::Down) => new_num = numindir + 1,
            _ => ()
        }
        if 0 <= ti && ti < width as i64 && 0 <= tj && tj < height as i64 && new_num <= 3 {
            ret.push((Coord { x: ti, y: tj }, d, new_num));
        }
    }
    ret
}

fn parse(s: &str) -> Vec<Vec<i64>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| 
                    c.to_string().parse::<i64>().unwrap() 
                )
                .collect()
        })
        .collect()
}

fn dijkstra(map: &Vec<Vec<i64>>, width: i64, height: i64) -> i64 {
    let mut pq = BinaryHeap::new();
    pq.push((-map[0][1] as i64, Coord { x: 1, y: 0 }, Direction::Right, 1));
    pq.push((-map[1][0] as i64, Coord { x: 0, y: 1 }, Direction::Down, 1));

    let mut seen = HashSet::new();

    while let Some((dist, curr, dir, num)) = pq.pop() {

        if (curr.x == width - 1) && (curr.y == height - 1) {
            return -dist;
        }
        // println!("{} {:?} {:?} {}", dist, curr, dir, num);

        for nbr in get_nbhrs(curr, width, height, dir, num) {
            // if -dist + (map[nbr.0.y][nbr.0.x] as i64) < dp[nbr.0.y][nbr.0.x] {
            //     dp[nbr.0.y][nbr.0.x] = -dist + (map[nbr.0.y][nbr.0.x] as i64);
            //     pq.push((-dp[nbr.0.y][nbr.0.x], nbr.0, nbr.1, nbr.2));
            // }
            // ADDING A FOURTH DIMENSION MADE THIS SO MUCH SLOWER
            if seen.insert((nbr.0, nbr.1, nbr.2)) {
                pq.push((dist - map[nbr.0.y as usize][nbr.0.x as usize], nbr.0, nbr.1, nbr.2));
            };
            
        }
    }
    -1
}

#[aoc(day17, part1)]
fn part1(input: &str) -> i64 {
    // Do Dijkstra
    let map = parse(input);
    let width = map[0].len() as i64;
    let height = map.len() as i64;

    dijkstra(&map, width, height)
}


fn get_nbhrs2(curr: Coord, width: i64, height: i64, map: &Vec<Vec<i64>>, dir: Direction, numindir: usize, dist: i64) -> Vec<(Coord, Direction, usize, i64)> {
    let mut ret = vec![];
    let Coord { x: ci, y: cj } = curr;
    for (ti, tj, d) in [(ci + 1, cj, Direction::Right), (ci - 1, cj, Direction::Left), (ci, cj + 1, Direction::Down), (ci, cj - 1, Direction::Up)] {
        let mut new_num = 4;
        match (dir, d) {
            (Direction::Left, Direction::Right) => continue,
            (Direction::Right, Direction::Left) => continue,
            (Direction::Up, Direction::Down) => continue,
            (Direction::Down, Direction::Up) => continue,
            (Direction::Left, Direction::Left) => new_num = numindir + 1,
            (Direction::Right, Direction::Right) => new_num = numindir + 1,
            (Direction::Up, Direction::Up) => new_num = numindir + 1,
            (Direction::Down, Direction::Down) => new_num = numindir + 1,
            _ => ()
        }
        if 0 <= ti && ti < width && 0 <= tj && tj < height && new_num <= 10 && new_num > 4 {
            ret.push((Coord { x: ti, y: tj }, d, new_num, dist - map[tj as usize][ti as usize]));
        } else if new_num == 4 {
            // Careful, Down is - and Up is +.
            // A smarter solution is: 
            // 1) Just increment in the current direction if new_num is less than 4, and then introduce a check on the final state
            // 2) Encapsulate all into a Crucible class
            // 3) Have a forward position class, instead of the hacky triplet solution above.
            // 4) Implement a Partial Order and Order yourself which is very clean instead of the min heap hack.
            match d {
                Direction::Right => if ci + 4 < width { ret.push((Coord { x: ci + 4, y: cj }, d, new_num, dist - map[cj as usize][(ci + 1) as usize..(ci + 5) as usize].iter().sum::<i64>())) },
                Direction::Left => if ci - 4 >= 0 { ret.push((Coord { x: ci - 4, y: cj }, d, new_num, dist - map[cj as usize][(ci - 4) as usize..(ci) as usize].iter().sum::<i64>())) },
                Direction::Down => if cj + 4 < height { ret.push((Coord { x: ci, y: cj + 4 }, d, new_num, dist - map[(cj + 1) as usize..(cj + 5) as usize].iter().map(|row| row[ci as usize]).sum::<i64>())) },
                Direction::Up => if cj - 4 >= 0 { ret.push((Coord { x: ci, y: cj - 4 }, d, new_num, dist - map[(cj - 4) as usize..cj as usize].iter().map(|row| row[ci as usize]).sum::<i64>())) },
            }
        }
    }
    ret
}

fn dijkstra2(map: &Vec<Vec<i64>>, width: i64, height: i64) -> i64 {
    let mut pq = BinaryHeap::new();
    pq.push((-map[0][1..5].iter().sum::<i64>() as i64, Coord { x: 4, y: 0 }, Direction::Right, 4));
    pq.push((-map[1..5].iter().map(|row| row[0]).sum::<i64>() as i64, Coord { x: 0, y: 4 }, Direction::Down, 4));
    // println!("{pq:?}");

    let mut seen = HashSet::new();

    while pq.len() > 0 {
        let (dist, curr, dir, num) = pq.pop().unwrap();
        // Found destination
        if (curr.x == width - 1) && (curr.y == height - 1) {
            return -dist;
        }
        // println!("{} {:?} {:?} {}", dist, curr, dir, num);

        for nbr in get_nbhrs2(curr, width, height, map, dir, num, dist) {
            if !seen.insert((nbr.0, nbr.1, nbr.2)) {
                continue;
            };
            pq.push((nbr.3, nbr.0, nbr.1, nbr.2));
        }
    }
    -1
}

// This code is INCOMPLETE and does not work.
#[aoc(day17, part2)]
fn part2(input: &str) -> i64 {
    let map = parse(input);
    let width = map[0].len() as i64;
    let height = map.len() as i64;

    dijkstra2(&map, width, height)
}
