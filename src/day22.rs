// use std::{ops::RangeInclusive, collections::{HashMap, HashSet}};

// use itertools::Itertools;


// enum Block {
//     X(usize, usize, RangeInclusive<usize>),
//     Y(usize, usize, RangeInclusive<usize>),
//     Z(usize, usize, RangeInclusive<usize>),
// }

// #[derive(Debug)]
// struct Line {
//     start: (usize, usize, usize),
//     end: (usize, usize, usize)
// }

// impl Line {
//     fn new(l: &str) -> Line {
//         // start: (usize, usize, usize), end: (usize, usize, usize)
//         let (left, right) = l.split_once("~").unwrap();
//         let mut start: Vec<usize> = left.split(",").map(|val| val.parse::<usize>().unwrap()).collect();
//         let mut end: Vec<usize> = right.split(",").map(|val| val.parse::<usize>().unwrap()).collect();
//         let mut should_swap = false;
//         for i in 0..start.len() {
//             if start[i] > end[i] {
//                 should_swap = true;
//             }
//         }
//         if should_swap {
//             let tmp = start;
//             start = end;
//             end = tmp;
//         }
//         Line { start: start.into_iter().take(3).collect_tuple().unwrap(), end: end.into_iter().take(3).collect_tuple().unwrap() }
//     }
// }

// #[aoc(day22, part1)]
// fn part1(input: &str) -> usize {
//     let mut lines = input.lines().map(|line| Line::new(line)).collect::<Vec<_>>();
//     lines.sort_by(|a, b| a.start.2.cmp(&b.start.2));

//     let mut levels: HashMap<(usize, usize), usize> = HashMap::new();
//     let mut ctob = HashMap::new();
//     let mut supports = vec![true; lines.len()];

//     for (idx, line) in lines.iter().enumerate() {
//         // println!("Line {}: {:?}", idx, line);
//         if line.start.2 != line.end.2 {

//             let w = (line.start.0, line.start.1);

//             if let Some(count) = levels.get(&w) {
//                 // println!("{:?} {:?} {:?}", w, line, count);
//                 match ctob.get(&(w.0, w.1, *count)) {
//                     None => {
//                         panic!("at the disco");
//                     },
//                     Some(idx) => {
//                         supports[*idx] = false;
//                     }
//                 }
//                 for z in *count..=*count + line.end.2 - line.start.2 + 1 {
//                     ctob.insert((line.start.0, line.start.1, z), idx);
//                 }
    
//                 levels.insert(w, count + line.end.2 - line.start.2 + 1);
//             } else {
//                 for z in 0..=line.end.2 - line.start.2 + 1 {
//                     ctob.insert((line.start.0, line.start.1, z), idx);
//                 }
//                 levels.insert(w, line.end.2 - line.start.2 + 1);
//             };

//         } else if line.start.1 != line.end.1 {
//             let mut tmp = vec![];
//             for y in line.start.1..=line.end.1 {
//                 match levels.get(&(line.start.0, y)) {
//                     Some(count) => tmp.push(*count),
//                     None => tmp.push(0),
//                 }
//             }
            
//             let max = tmp.iter().max().unwrap();
//             let mut under = HashSet::new();
//             let mut last = 0;

//             for y in line.start.1..=line.end.1 { 
//                 ctob.insert((line.start.0, y, max + 1), idx);
//                 levels.insert((line.start.0, y), max + 1);
//                 match ctob.get(&(line.start.0, y, *max)) {
//                     Some(idx) => {
//                         under.insert(*idx);
//                         last = *idx;
//                     },
//                     None => (),
//                 };
//             } 
//             if under.len() == 1 {
//                 supports[last] = false;
//             } else if under.len() == 0 && *max != 0 {
//                 panic!("at the disco1");
//             }
//         } else if line.start.0 != line.end.0 {
//             let mut tmp = vec![];
//             for x in line.start.0..=line.end.0 {
//                 match levels.get(&(x, line.start.1)) {
//                     Some(count) => tmp.push(*count),
//                     None => tmp.push(0),
//                 }
//             }

//             let max = tmp.iter().max().unwrap();
//             let mut under = HashSet::new();
//             let mut last = 0;

//             for x in line.start.0..=line.end.0 { 
//                 ctob.insert((x, line.start.1, max + 1), idx);
//                 levels.insert((x, line.start.1), max + 1);
//                 match ctob.get(&(x, line.start.1, *max)) {
//                     Some(idx) => {
//                         under.insert(*idx);
//                         last = *idx;
//                     },
//                     None => (),
//                 };
//             } 
//             if under.len() == 1 {
//                 supports[last] = false;
//             }
//             else if under.len() == 0 && *max != 0 {
//                 panic!("at the disco2");
//             }
//         } 
//     }

//     lines.len() - supports.iter().filter(|s| !**s).count()
// }

// #[aoc(day22, part2)]
// fn part2(input: &str) -> usize {
//     0
// }

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[aoc(day22, part1)]
fn p1(input: &str) -> usize {
    let bricks = setup(input);
    bricks
        .values()
        .filter(|br| br.above.is_empty() || br.above.iter().all(|b| bricks[b].below.len() > 1))
        .count()
}

#[aoc(day22, part2)]
fn p2(input: &str) -> usize {
    let bricks = setup(input);
    // SUM!! not max
    bricks.keys().map(|&id| disintegrate(id, &bricks)).sum()
}

fn disintegrate(start: u32, bricks: &HashMap<u32, Brick>) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(id) = queue.pop_front() {
        if seen.contains(&id) {
            continue;
        }
        seen.insert(id);
        queue.extend(bricks[&id].above.iter().filter_map(|a| {
            if bricks[a].below.is_subset(&seen) {
                Some(*a)
            } else {
                None
            }
        }))
    }
    seen.len().saturating_sub(1)
}

fn setup(input: &str) -> HashMap<u32, Brick> {
    let mut grid = HashMap::new();
    let mut bricks = HashMap::new();
    // SORT!!
    for brick in parse(input).sorted_by_key(|b| b.zs) {
        brick.settle(&mut grid, &mut bricks)
    }
    bricks
}

fn parse(input: &str) -> impl Iterator<Item = Brick> + '_ {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| Brick::new(i as u32, line))
}

#[derive(Debug, Clone)]
struct Brick {
    id: u32,
    xs: (u32, u32),
    ys: (u32, u32),
    zs: (u32, u32),
    above: HashSet<u32>,
    below: HashSet<u32>,
}

impl Brick {
    fn new(id: u32, line: &str) -> Self {
        let mut nums = [0; 6];
        for (i, s) in line.trim().split(&[',', '~']).enumerate() {
            nums[i] = s.parse().unwrap();
        }
        Self {
            id,
            xs: (nums[0], nums[3]),
            ys: (nums[1], nums[4]),
            zs: (nums[2], nums[5]),
            above: HashSet::new(),
            below: HashSet::new(),
        }
    }

    fn settle(
        mut self,
        grid: &mut HashMap<u32, Vec<(u32, u32, u32)>>,
        bricks: &mut HashMap<u32, Brick>,
    ) {
        let mut z = self.zs.0 - 1;
        let mut below = HashSet::new();
        while z > 0 {
            if let Some(plain) = grid.get(&z) {
                below = plain
                    .iter()
                    .filter_map(|p| {
                        if (self.xs.0..=self.xs.1).contains(&p.0)
                            && (self.ys.0..=self.ys.1).contains(&p.1)
                        {
                            Some(p.2)
                        } else {
                            None
                        }
                    })
                    .collect();
                if !below.is_empty() {
                    break;
                }
            }
            z -= 1;
        }
        let z1 = z + 1;
        let z2 = self.zs.1 - (self.zs.0 - z1);
        let points = (self.xs.0..=self.xs.1)
            .cartesian_product(self.ys.0..=self.ys.1)
            .map(|(x, y)| (x, y, self.id))
            .collect::<Vec<_>>();

        for z in z1..=z2 {
            grid.entry(z)
                .and_modify(|v| v.extend(points.clone()))
                .or_insert(points.clone());
        }
        for id in &below {
            if let Some(brick) = bricks.get_mut(id) {
                brick.above.insert(self.id);
            }
        }
        self.zs = (z1, z2);
        self.below = below;
        bricks.insert(self.id, self);
    }
}
