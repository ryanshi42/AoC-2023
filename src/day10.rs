use core::panic;
use std::collections::{HashSet};

use crate::day10::FromDir::*;

pub enum FromDir {
    Top,
    Bottom,
    Left,
    Right
}

fn get_new_pos(ppos: (usize, usize), pdir: FromDir, grid: &Vec<&str>) -> ((usize, usize), FromDir) {
    
    // As I said before there is a way better implementation that I couldn't be bothered implementing
    let l = grid[ppos.0].chars().nth(ppos.1).unwrap();
    match (l, pdir) {
        ('|', Top) => ((ppos.0 + 1, ppos.1), Top), 
        ('|', Bottom) => ((ppos.0 - 1, ppos.1), Bottom),
        ('-', Left) => ((ppos.0, ppos.1 + 1), Left),
        ('-', Right) => ((ppos.0, ppos.1 - 1), Right),
        ('L', Top) => ((ppos.0, ppos.1 + 1), Left),
        ('L', Right) => ((ppos.0 - 1, ppos.1), Bottom),
        ('7', Left) => ((ppos.0 + 1, ppos.1), Top),
        ('7', Bottom) => ((ppos.0, ppos.1 - 1), Right),
        ('J', Top) => ((ppos.0, ppos.1 - 1), Right),
        ('J', Left) => ((ppos.0 - 1, ppos.1), Bottom),
        ('F', Right) => ((ppos.0 + 1, ppos.1), Top),
        ('F', Bottom) => ((ppos.0, ppos.1 + 1), Left),
        _ => {
            panic!("wtf");
        }
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> i32 {
    let mut ans = 0;

    let grid: Vec<&str> = input.lines().collect();
    let mut si = 0;
    let mut sj = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, elem) in (*line).chars().enumerate() {
            if elem == 'S' {
                si = i;
                sj = j;
                break;
            }
        }
    }

    // Manual inspect:
    // There exists a WAY better implementation where you map direction to the cardinal direction but I'm lazy
    let mut lpos = (si, sj + 1);
    let mut rpos = (si + 1, sj);
    let mut rfrom = FromDir::Top; 
    let mut lfrom = FromDir::Left; 

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    while seen.get(&lpos).is_none() && seen.get(&rpos).is_none() {
        ans += 1;
        seen.insert(lpos);
        seen.insert(rpos);

        (lpos, lfrom) = get_new_pos(lpos, lfrom, &grid);
        (rpos, rfrom) = get_new_pos(rpos, rfrom, &grid);

    } 

    return ans;
    

}

#[aoc(day10, part2)]
fn part2(input: &str) -> i32 {
    let grid: Vec<&str> = input.lines().collect();
    let mut si = 0;
    let mut sj = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, elem) in (*line).chars().enumerate() {
            if elem == 'S' {
                si = i;
                sj = j;
                break;
            }
        }
    }

    // Manual inspect:
    let mut lpos = (si, sj + 1);
    let mut rpos = (si + 1, sj);
    let mut rfrom = FromDir::Top; 
    let mut lfrom = FromDir::Left; 

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert((si, sj));

    let mut vertices = vec![];
    vertices.push((si, sj));

    while seen.get(&lpos).is_none() && seen.get(&rpos).is_none() {
        seen.insert(lpos);
        seen.insert(rpos);
        let mut l = grid[lpos.0].chars().nth(lpos.1).unwrap();
        if l != '|' && l != '-' {
            vertices.push(lpos);
        }
        l = grid[rpos.0].chars().nth(rpos.1).unwrap();
        if l != '|' && l != '-' {
            vertices.push(rpos);
        }

        (lpos, lfrom) = get_new_pos(lpos, lfrom, &grid);
        (rpos, rfrom) = get_new_pos(rpos, rfrom, &grid);


    } 
    seen.insert(lpos);
    seen.insert(rpos);
    let mut ans = 0;
    for (i, line) in grid.iter().enumerate() {
        let mut inside = false;
        ans += (*line).chars().enumerate().filter(|item| match item.1 {
            '|' | 'J' | 'L' => {
                if seen.get(&(i, item.0)).is_some() {
                    inside = !inside;
                    false
                } else {
                    inside
                }
            }
            _ => {
                if seen.get(&(i, item.0)).is_some() {
                    false
                } else {
                    inside
                }
            },
        })
        .count();
    }

    return ans as i32;
    // let coords =  
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        // assert_eq!(part1(c), 114);
    }
}