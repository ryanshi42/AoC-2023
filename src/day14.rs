use std::{collections::{HashMap}};

fn convert(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {

    let x = transpose(convert(input));
    let mut ans = 0;

    let mut orocks = vec![];
    let mut hrocks = vec![];
    for (i, line) in x.iter().enumerate() {
        let mut k = x.len();
        for (j, elem) in line.iter().enumerate() {
            if *elem == 'O' {
                ans += k;
                k -= 1;
                orocks.push((i, j));
            } else if *elem == '#' {
                k = x.len() - j - 1;
                hrocks.push((i, j));
            }
        }
    }



    ans as u64
    // input.split("\n\n")
    //     .map(|x| f(x))
    //     .sum()
}

fn g(x: Vec<Vec<char>>, up: bool) -> Vec<Vec<char>> {

    let mut y = x.clone();
    if !up {
        y = x.clone().into_iter().map(|row| row.into_iter().rev().collect()).collect();
    }
    let mut new = y.clone();
    for (i, line) in y.iter().enumerate() {
        let mut k = 0;
        for (j, elem) in line.iter().enumerate() {
            if *elem == 'O' {
                new[i][k] = 'O';
                if k != j {
                    new[i][j] = '.';
                }
                k += 1;
            } else if *elem == '#' {
                new[i][j] = '#';
                k = j + 1;
            } else {
                new[i][j] = '.';
            }
        }
    }
    if !up {
        new = new.into_iter().map(|row| row.into_iter().rev().collect()).collect();
    }
    new
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut x = convert(input);
    let mut ans = 0;

    let mut hs:HashMap<Vec<Vec<char>>, i32> = HashMap::new();
    for i in 1..1000000001 {
        let z1 = g(transpose(x.clone()), true);
        let z2 = g(transpose(z1), true);
        let z3 = g(transpose(z2), false);
        let z4 = g(transpose(z3), false);
        if x == z4 {
            break;
        }
        x = z4;
        if hs.get(&x).is_some() {
            let start = hs.get(&x).unwrap();
            let cycle = i - start;
            let l = ((1000000000 - start) % cycle) + start;
            for (lmao, v) in &hs {
                // println!("{l} {cycle} {}", *v);
                if *v == l {
                    for (i, line) in lmao.iter().enumerate() {
                        for (_j, elem) in line.iter().enumerate() {
                            if *elem == 'O' {
                                ans += x.len() - i;
                            } 
                        }
                    }
                    break;
                }
            }
            // println!("Cycle starts at {}", start);
            break;
        }
        hs.insert(x.clone(), i);
    }
    ans as u64

}