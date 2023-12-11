use std::{cmp::{max, min}};

use num::abs;

#[aoc(day11, part1)]
fn part1(input: &str) -> i32 {
    let mut ans = 0;

    let mut grid = vec![];
    for line in input.lines() {
        grid.push(line);
        if line.chars().all(|x| x == '.') {
            grid.push(line);
        }
    }

    let mut se = vec![];
    for i in 0..grid[0].len() {
        let mut temp = vec![];
        for line in input.lines() {
            temp.push(line.chars().nth(i));
        }
        if temp.iter().all(|x| x.unwrap() == '.') {
            se.push(i);
        }
    }

    let mut gs = vec![];
    for (i, line) in grid.iter().enumerate() {
        for (j, elem) in line.chars().enumerate() {
            if elem == '#' {
                gs.push((i, j));
            }
        } 
    }
    // println!("{se:?}");

    for (i, x) in gs.iter().enumerate() {
        for (j, y) in gs.iter().enumerate() {
            if j <= i {
                continue;
            }
            let a = min(x.1, y.1);
            let b = max(x.1, y.1);

            let new = se.clone().into_iter().filter(|c| *c > a && *c < b).count() as i32 + abs(x.0 as i32 - y.0 as i32) + abs(x.1 as i32 - y.1 as i32);
            // println!("{x:?} {y:?} {new}");
            ans += new;
        }
    }
    return ans;
}

#[aoc(day11, part2)]
fn part2(input: &str) -> i32 {
    let mut ans = 0;

    // Determine a list of columns and rows which must be expanded
    let grid: Vec<&str> = input.lines().collect(); 
    let mut sw = vec![]; 
    for (i, line) in input.lines().enumerate() {
        if line.chars().all(|x| x == '.') {
            sw.push(i);
        }
    }

    let mut se = vec![];
    for i in 0..grid[0].len() {
        let mut temp = vec![];
        for line in input.lines() {
            temp.push(line.chars().nth(i));
        }
        if temp.iter().all(|x| x.unwrap() == '.') {
            se.push(i);
        }
    }

    // Get list of galaxies
    let mut gs = vec![];
    for (i, line) in grid.iter().enumerate() {
        for (j, elem) in line.chars().enumerate() {
            if elem == '#' {
                gs.push((i, j));
            }
        } 
    }
    // println!("{se:?}");
    // println!("{sw:?}");

    // Got pretty lucky that the part 2 was quite similar to part 1, and couldn't be brute forced.
    for (i, x) in gs.iter().enumerate() {
        for (j, y) in gs.iter().enumerate() {
            if j <= i {
                continue;
            }
            let a = min(x.1, y.1);
            let b = max(x.1, y.1);
            let c = min(x.0, y.0);
            let d = max(x.0, y.0);

            let new = sw.clone().into_iter().filter(|e| *e > c && *e < d).count() as i64 * 999999 + se.clone().into_iter().filter(|z| *z > a && *z < b).count() as i64 * 999999 + abs(x.0 as i64 - y.0 as i64) + abs(x.1 as i64 - y.1 as i64);
            // println!("{x:?} {y:?} {new}");
            ans += new;
        }
    }
    // stupid overflow errors
    println!("{ans}");
    return ans as i32;
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        // assert_eq!(part1(c), 114);
    }
}